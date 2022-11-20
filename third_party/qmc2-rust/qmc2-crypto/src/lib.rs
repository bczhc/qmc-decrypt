mod crypto;

pub use crypto::detection;
pub use crypto::errors;
pub use crypto::key_dec::*;
pub use crypto::qmc2::decrypt_factory;
pub use crypto::qmc2_base::QMC2Crypto;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
