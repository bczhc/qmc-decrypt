use super::qmc2_base::QMC2Crypto;

const FIRST_SEGMENT_SIZE: usize = 0x80;
const OTHER_SEGMENT_SIZE: usize = 0x1400;

/// Recommends 2.5M block, aligns to the segment size.
const RECOMMENDED_BLOCK_SIZE: usize = (1024 * 1024) * 5 / 2;
static_assertions::const_assert_eq!(RECOMMENDED_BLOCK_SIZE % OTHER_SEGMENT_SIZE, 0);

pub struct QMCStreamRC4Crypto {
    /// RC4 seed box
    s: Vec<u8>,
    /// Hash base, used to calculate some other key
    hash: u32,
    /// RC4 key, though is used different than a standard RC4 algorithm...
    rc4_key: Vec<u8>,
}

impl QMCStreamRC4Crypto {
    #[inline]
    pub(self) fn calc_segment_key(&self, id: usize, seed: u8) -> usize {
        let dividend = f64::from(self.hash);
        let divisor = ((id + 1) * usize::from(seed)) as f64;
        let key = dividend / divisor * 100.0;
        key as u64 as usize
    }

    #[inline]
    /// Get next rc4 xor byte value
    pub(self) fn rc4_derive(n: usize, s: &mut Vec<u8>, j: &mut usize, k: &mut usize) -> u8 {
        *j = (*j + 1) % n;
        *k = (usize::from(s[*j]) + *k) % n;

        s.swap(*j, *k);

        let index = usize::from(s[*j]) + usize::from(s[*k]);
        s[index % n]
    }

    #[inline]
    /// Encode first segment
    pub(self) fn encode_first_segment(&self, offset: usize, buf: &mut [u8]) {
        let n = self.rc4_key.len();
        let mut offset = offset;
        for b in buf.iter_mut() {
            let key1 = self.rc4_key[offset % n];
            let key2 = self.calc_segment_key(offset, key1);
            *b ^= self.rc4_key[key2 % n];

            offset += 1;
        }
    }

    #[inline]
    /// Encode segments (other than the first one)
    pub(self) fn encode_other_segment(&self, offset: usize, buf: &mut [u8]) {
        // segment_id: 0~511 (inclusive)
        let seg_id = offset / OTHER_SEGMENT_SIZE;
        let seg_id_small = seg_id & 0x1FF;

        let mut discard_count = self.calc_segment_key(seg_id, self.rc4_key[seg_id_small]) & 0x1FF;
        discard_count += offset % OTHER_SEGMENT_SIZE;

        let n = self.rc4_key.len();
        let mut s = self.s.clone();
        let mut j = 0usize;
        let mut k = 0usize;
        for _ in 0..discard_count {
            QMCStreamRC4Crypto::rc4_derive(n, &mut s, &mut j, &mut k);
        }

        for b in buf.iter_mut() {
            *b ^= QMCStreamRC4Crypto::rc4_derive(n, &mut s, &mut j, &mut k);
        }
    }

    #[inline]
    pub(self) fn calc_hash_base(data: &[u8]) -> u32 {
        let mut hash: u32 = 1;

        for &value in data.iter() {
            let value = u32::from(value);

            // Skip if the next byte is zero.
            if value == 0 {
                continue;
            }

            // Naive overflow check - keeping as it is to maintain compatibility.
            let next_hash = hash.wrapping_mul(value);
            if next_hash == 0 || next_hash <= hash {
                break;
            }

            hash = next_hash;
        }

        hash
    }
}

impl QMCStreamRC4Crypto {
    pub fn new(rc4_key: &[u8]) -> Self {
        // n == rc4_key.len() == s.len()
        let n = rc4_key.len();
        let mut s = vec![0u8; n];
        for (i, b) in s.iter_mut().enumerate() {
            *b = i as u8;
        }

        let mut j = 0usize;
        for (i, &key) in rc4_key.iter().enumerate() {
            j = j.wrapping_add(s[i] as usize).wrapping_add(key as usize) % n;
            s.swap(i, j);
        }

        QMCStreamRC4Crypto {
            s,
            hash: QMCStreamRC4Crypto::calc_hash_base(rc4_key),
            rc4_key: rc4_key.to_vec(),
        }
    }
}

impl QMC2Crypto for QMCStreamRC4Crypto {
    fn get_recommended_block_size(&self) -> usize {
        RECOMMENDED_BLOCK_SIZE
    }

    fn decrypt(&self, offset: usize, buf: &mut [u8]) {
        let mut offset = offset;
        let mut len = buf.len();
        let mut i = 0usize;

        // First segment have a different algorithm.
        if offset < FIRST_SEGMENT_SIZE {
            let len_processed = std::cmp::min(len, FIRST_SEGMENT_SIZE - offset);
            self.encode_first_segment(offset, &mut buf[i..i + len_processed]);
            i += len_processed;
            len -= len_processed;
            offset += len_processed;
        }

        // Align a segment
        let to_align = offset % OTHER_SEGMENT_SIZE;
        if to_align != 0 {
            let len_processed = std::cmp::min(len, OTHER_SEGMENT_SIZE - to_align);
            self.encode_other_segment(offset, &mut buf[i..i + len_processed]);
            i += len_processed;
            len -= len_processed;
            offset += len_processed;
        }

        // Process segments
        while len > OTHER_SEGMENT_SIZE {
            self.encode_other_segment(offset, &mut buf[i..i + OTHER_SEGMENT_SIZE]);
            i += OTHER_SEGMENT_SIZE;
            len -= OTHER_SEGMENT_SIZE;
            offset += OTHER_SEGMENT_SIZE;
        }

        // Left over
        if len > 0 {
            self.encode_other_segment(offset, &mut buf[i..i + len]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_hash_base() {
        let hash = QMCStreamRC4Crypto::calc_hash_base(&[1u8, 99]);
        assert_eq!(hash, 1);

        let hash = QMCStreamRC4Crypto::calc_hash_base(&[
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // 8
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // 16
        ]);
        assert_eq!(hash, 0xfc05fc01);

        // should ignore zeros
        let hash = QMCStreamRC4Crypto::calc_hash_base(&[
            0x00, //
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, //
            0x00, //
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, //
        ]);
        assert_eq!(hash, 0xfc05fc01);
    }

    #[test]
    fn test_decrypt_first_segment() {
        let mut rc4_key = [0u8; 255];
        for (i, p) in rc4_key.iter_mut().enumerate() {
            *p = i as u8
        }
        let crypto = QMCStreamRC4Crypto::new(&rc4_key);
        let mut data = [0u8; 16];
        crypto.decrypt(0, &mut data);
        assert_eq!(data, [0, 50, 16, 8, 5, 3, 2, 1, 1, 1, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_decrypt_between_first_and_second_segment() {
        let mut rc4_key = [0u8; 255];
        for (i, p) in rc4_key.iter_mut().enumerate() {
            *p = i as u8
        }
        let crypto = QMCStreamRC4Crypto::new(&rc4_key);
        let mut data = [0u8; 16];
        crypto.decrypt(FIRST_SEGMENT_SIZE - 8, &mut data);
        assert_eq!(
            data,
            [
                0u8, 0, 0, 0, 0, 0, 0, 0, // first segment
                141, 97, 122, 193, 166, 101, 233, 214, // after the "first" segment
            ]
        );
    }

    #[test]
    fn test_decrypt_between_2_segments() {
        let mut rc4_key = [0u8; 255];
        for (i, p) in rc4_key.iter_mut().enumerate() {
            *p = i as u8
        }
        let crypto = QMCStreamRC4Crypto::new(&rc4_key);
        let mut data = [0u8; 16];
        crypto.decrypt(OTHER_SEGMENT_SIZE - 8, &mut data);
        assert_eq!(
            data,
            [
                118, 193, 176, 83, 10, 98, 105, 234, // end of first "other" segment
                151, 56, 198, 1, 226, 173, 127, 4, // second "other" segment
            ]
        );
    }

    #[test]
    fn test_decrypt_second_segment() {
        let mut rc4_key = [0u8; 255];
        for (i, p) in rc4_key.iter_mut().enumerate() {
            *p = i as u8
        }
        let crypto = QMCStreamRC4Crypto::new(&rc4_key);
        let mut data = [0u8; 16];
        crypto.decrypt(OTHER_SEGMENT_SIZE, &mut data);
        assert_eq!(
            data,
            [
                151, 56, 198, 1, 226, 173, 127, 4, // beginning of second "other" segment
                181, 165, 171, 21, 82, 152, 195, 210, // next 8 bytes, same segment
            ]
        );
    }

    #[test]
    fn test_decrypt_entire_segment() {
        let mut rc4_key = [0u8; 255];
        for (i, p) in rc4_key.iter_mut().enumerate() {
            *p = i as u8
        }
        let crypto = QMCStreamRC4Crypto::new(&rc4_key);
        let mut data = vec![0u8; OTHER_SEGMENT_SIZE + 1];
        crypto.decrypt(OTHER_SEGMENT_SIZE, &mut data);
        // Only checks for the first 16 bytes
        assert_eq!(
            data[0..16],
            [
                151, 56, 198, 1, 226, 173, 127, 4, // beginning of second "other" segment
                181, 165, 171, 21, 82, 152, 195, 210, // next 8 bytes, same segment
            ]
        );
    }
}
