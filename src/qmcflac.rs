pub struct Cipher {
    x: i64,
    y: i64,
    i: i64,
    dx: i64,
}

impl Default for Cipher {
    fn default() -> Self {
        Self {
            x: -1,
            y: 8,
            dx: 1,
            i: -1,
        }
    }
}

impl Cipher {
    pub fn new() -> Self {
        Self::default()
    }

    const SEED_MAP: [[u8; 7]; 8] = [
        [0x4a, 0xd6, 0xca, 0x90, 0x67, 0xf7, 0x52],
        [0x5e, 0x95, 0x23, 0x9f, 0x13, 0x11, 0x7e],
        [0x47, 0x74, 0x3d, 0x90, 0xaa, 0x3f, 0x51],
        [0xc6, 0x09, 0xd5, 0x9f, 0xfa, 0x66, 0xf9],
        [0xf3, 0xd6, 0xa1, 0x90, 0xa0, 0xf7, 0xf0],
        [0x1d, 0x95, 0xde, 0x9f, 0x84, 0x11, 0xf4],
        [0x0e, 0x74, 0xbb, 0x90, 0xbc, 0x3f, 0x92],
        [0x00, 0x09, 0x5b, 0x9f, 0x62, 0x66, 0xa1],
    ];
}

impl Cipher {
    #[inline]
    fn next_mask(&mut self) -> u8 {
        let mut ret: u8;
        loop {
            self.i += 1;
            if self.x < 0 {
                self.dx = 1;
                self.y = (8 - self.y) % 8;
                ret = 0xc3;
            } else if self.x > 6 {
                self.dx = -1;
                self.y = 7 - self.y;
                ret = 0xd8;
            } else {
                ret = Self::SEED_MAP[self.y as usize][self.x as usize];
            }
            self.x += self.dx;
            if !(self.i == 0x8000 || (self.i > 0x8000 && ((self.i + 1) % 0x8000) == 0)) {
                break;
            }
        }
        ret
    }

    pub fn process(&mut self, buf: &mut [u8]) {
        for b in buf {
            *b ^= self.next_mask();
        }
    }
}

pub mod read {
    use std::io::Read;

    use super::Cipher;

    /// Read-based stream
    pub struct Stream<R>
    where
        R: Read,
    {
        cipher: Cipher,
        reader: R,
    }

    impl<R> Stream<R>
    where
        R: Read,
    {
        pub fn new(reader: R) -> Self {
            Self {
                cipher: Cipher::new(),
                reader,
            }
        }
    }

    impl<R> Read for Stream<R>
    where
        R: Read,
    {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            let size = self.reader.read(buf)?;
            self.cipher.process(buf);
            Ok(size)
        }
    }
}
