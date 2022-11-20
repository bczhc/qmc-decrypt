use super::errors::CryptoError;
use super::key_dec;
use super::qmc2_base::QMC2Crypto;
use super::qmc2_map::QMCStreamMapCrypto;
use super::qmc2_rc4::QMCStreamRC4Crypto;

pub fn decrypt_factory(ekey: &str) -> Result<Box<dyn QMC2Crypto>, CryptoError> {
    let key = key_dec::parse_ekey(ekey)?;

    // use RC4 if > 300, otherwise use old xor algorithm.
    Ok(if key.len() > 300 {
        Box::new(QMCStreamRC4Crypto::new(&key))
    } else {
        Box::new(QMCStreamMapCrypto::new(&key))
    })
}
