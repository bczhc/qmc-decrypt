pub mod read {
    use std::io::Read;

    pub struct Stream<R>
    where
        R: Read,
    {
        reader: R,
        crypto: Box<dyn qmc2_crypto::QMC2Crypto>,
        offset: u64,
    }

    impl<R> Stream<R>
    where
        R: Read,
    {
        pub fn new(reader: R, ekey: &str) -> Result<Self, qmc2_crypto::errors::CryptoError> {
            let crypto = qmc2_crypto::decrypt_factory(ekey)?;
            Ok(Self {
                crypto,
                reader,
                offset: 0,
            })
        }
    }

    impl<R> Read for Stream<R>
    where
        R: Read,
    {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            let size = self.reader.read(buf)?;
            self.crypto.decrypt(self.offset as usize, &mut buf[..size]);
            Ok(size)
        }
    }
}
