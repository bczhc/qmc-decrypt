pub struct Cipher;

impl Cipher {
    pub fn process(offset: u64, buf: &mut [u8]) {
        for (i, b) in buf.iter_mut().enumerate() {
            let pos = offset + i as u64;
            let mut index = (pos % 0x7fff) & 0x7f;
            if index > 0x3f {
                index = (0x80 - index) & 0x3f;
            }
            *b ^= Self::KEYS[index as usize];
        }
    }
}

impl Cipher {
    const KEYS: [u8; 64] = [
        0xc3, 0x4a, 0xd6, 0xca, 0x90, 0x67, 0xf7, 0x52, 0xd8, 0xa1, 0x66, 0x62, 0x9f, 0x5b, 0x09,
        0x00, 0xc3, 0x5e, 0x95, 0x23, 0x9f, 0x13, 0x11, 0x7e, 0xd8, 0x92, 0x3f, 0xbc, 0x90, 0xbb,
        0x74, 0x0e, 0xc3, 0x47, 0x74, 0x3d, 0x90, 0xaa, 0x3f, 0x51, 0xd8, 0xf4, 0x11, 0x84, 0x9f,
        0xde, 0x95, 0x1d, 0xc3, 0xc6, 0x09, 0xd5, 0x9f, 0xfa, 0x66, 0xf9, 0xd8, 0xf0, 0xf7, 0xa0,
        0x90, 0xa1, 0xd6, 0xf3,
    ];
}

pub mod read {
    use std::io::Read;

    use super::Cipher;

    /// Read-based stream
    pub struct Stream<R>
    where
        R: Read,
    {
        reader: R,
        offset: u64,
    }

    impl<R> Stream<R>
    where
        R: Read,
    {
        pub fn new(reader: R) -> Self {
            Self { reader, offset: 0 }
        }
    }

    impl<R> Read for Stream<R>
    where
        R: Read,
    {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            let size = self.reader.read(buf)?;
            Cipher::process(self.offset, buf);
            self.offset += size as u64;
            Ok(size)
        }
    }
}
