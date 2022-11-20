use super::qmc2_base::QMC2Crypto;

/// Recommends 2M block. No preference.
const RECOMMENDED_BLOCK_SIZE: usize = 2 * 1024 * 1024;

pub struct QMCStreamMapCrypto {
    key: Vec<u8>,
}

impl QMCStreamMapCrypto {
    pub fn new(key: &[u8]) -> Self {
        QMCStreamMapCrypto { key: key.to_vec() }
    }

    #[inline]
    /// Last step of the key derivation, scramble the value by its key used.
    pub(self) fn scramble_by_index(value: u8, index: usize) -> u8 {
        let rotation = (index as u32).wrapping_add(4) & 0b111;

        let left = value.wrapping_shl(rotation);
        let right = value.wrapping_shr(rotation);

        left | right
    }

    #[inline]
    pub(self) fn map_l(&self, offset: usize) -> u8 {
        let mut offset_local = offset;

        if offset_local > 0x7FFF {
            offset_local %= 0x7FFF;
        }

        let index = (offset_local * offset_local + 71214) % self.key.len();
        QMCStreamMapCrypto::scramble_by_index(self.key[index], index)
    }
}

impl QMC2Crypto for QMCStreamMapCrypto {
    fn get_recommended_block_size(&self) -> usize {
        RECOMMENDED_BLOCK_SIZE
    }

    fn decrypt(&self, offset: usize, buf: &mut [u8]) {
        buf.iter_mut().enumerate().for_each(|(i, byte)| {
            *byte ^= self.map_l(offset + i);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEY: [u8; 16] = [
        0x41u8, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, //
        0x49u8, 0x4A, 0x4B, 0x4C, 0x4D, 0x4E, 0x4F, 0x50, //
    ];
    const EXPECTED1: [u8; 16] = [
        0x3Fu8, 0x8A, 0xC1, 0x49, 0x3F, 0x49, 0xC1, 0x8A, //
        0x3Fu8, 0x8A, 0xC1, 0x49, 0x3F, 0x49, 0xC1, 0x8A, //
    ];
    const EXPECTED2: [u8; 16] = [
        0x8Au8, 0x3F, 0x8A, 0xC1, 0x49, 0x3F, 0x49, 0xC1, //
        0x8Au8, 0x8A, 0xC1, 0x49, 0x3F, 0x49, 0xC1, 0x8A, //
    ];

    #[test]
    fn map_l_test_1() {
        let crypto = QMCStreamMapCrypto::new(&KEY);
        let mut data = [0u8; 16];
        crypto.decrypt(0, &mut data);
        assert_eq!(data, EXPECTED1);
    }

    #[test]
    fn map_l_test_boundary() {
        let crypto = QMCStreamMapCrypto::new(&KEY);
        let mut data = [0u8; 16];
        crypto.decrypt(0x7FFF - 8, &mut data);
        assert_eq!(data, EXPECTED2);
    }
}
