pub trait QMC2Crypto {
    fn get_recommended_block_size(&self) -> usize;
    fn decrypt(&self, offset: usize, buf: &mut [u8]);
}
