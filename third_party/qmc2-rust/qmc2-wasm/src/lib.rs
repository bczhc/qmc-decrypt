mod utils;

use qmc2_crypto as qmc2;
use qmc2_crypto::detection::Detection;
use qmc2_crypto::QMC2Crypto;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct DetectionWrapper {
    #[wasm_bindgen]
    pub eof_position: i32,
    #[wasm_bindgen]
    pub ekey_position: i32,
    #[wasm_bindgen]
    pub ekey_len: usize,
    song_id: String,
}

impl DetectionWrapper {
    pub(crate) fn from(d: Detection) -> Self {
        DetectionWrapper {
            eof_position: d.eof_position as i32,
            ekey_position: d.ekey_position as i32,
            ekey_len: d.ekey_len,
            song_id: d.song_id,
        }
    }
}

#[wasm_bindgen]
impl DetectionWrapper {
    #[wasm_bindgen]
    pub fn get_song_id(&self) -> String {
        self.song_id.as_str().into()
    }
}

#[wasm_bindgen]
pub fn get_recommended_detection_size() -> usize {
    qmc2::detection::RECOMMENDED_DETECTION_SIZE
}

#[wasm_bindgen(catch)]
pub fn detect(buf: &[u8]) -> Result<DetectionWrapper, JsValue> {
    qmc2::detection::detect(buf)
        .map(DetectionWrapper::from)
        .map_err(|e| JsValue::from(e.to_string()))
}

#[wasm_bindgen]
pub struct QMC2CryptoWrapper(Box<dyn QMC2Crypto>);

#[wasm_bindgen]
impl QMC2CryptoWrapper {
    #[wasm_bindgen]
    pub fn get_recommended_block_size(&self) -> usize {
        self.0.get_recommended_block_size()
    }

    #[wasm_bindgen]
    pub fn decrypt(&self, offset: usize, buf: &mut [u8]) {
        self.0.decrypt(offset, buf)
    }
}

#[wasm_bindgen(catch)]
pub fn decrypt_factory(ekey: String) -> Result<QMC2CryptoWrapper, JsValue> {
    qmc2::decrypt_factory(ekey.as_str())
        .map(QMC2CryptoWrapper)
        .map_err(|e| JsValue::from(e.to_string()))
}

#[wasm_bindgen]
pub fn __init() {
    utils::set_panic_hook();
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b - 1
}
