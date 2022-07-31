use crate::WatermarkedImage;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct WatermarkedImageWeb {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) format: String,
    pub(crate) original: Uint8Array,
    pub(crate) watermarked: Uint8Array,
}

#[wasm_bindgen]
impl WatermarkedImageWeb {
    #[wasm_bindgen(constructor)]
    pub fn new(
        width: u32,
        height: u32,
        format: String,
        original: Uint8Array,
        watermarked: Uint8Array,
    ) -> WatermarkedImageWeb {
        WatermarkedImageWeb {
            width,
            height,
            format,
            original,
            watermarked,
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    pub fn get_format(&self) -> String {
        self.format.clone()
    }

    pub fn set_format(&mut self, format: String) {
        self.format = format;
    }

    pub fn get_original(&self) -> Uint8Array {
        self.original.clone()
    }

    pub fn set_original(&mut self, original: Uint8Array) {
        self.original = original;
    }

    pub fn get_watermarked(&self) -> Uint8Array {
        self.watermarked.clone()
    }

    pub fn set_watermarked(&mut self, watermarked: Uint8Array) {
        self.watermarked = watermarked;
    }
}
