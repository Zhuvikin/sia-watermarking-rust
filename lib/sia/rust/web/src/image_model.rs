use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct ImageModel {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) data: Uint8Array,
}

#[wasm_bindgen]
impl ImageModel {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, data: Uint8Array) -> ImageModel {
        ImageModel {
            width,
            height,
            data,
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

    pub fn get_data(&self) -> Uint8Array {
        self.data.clone()
    }

    pub fn set_data(&mut self, data: Uint8Array) {
        self.data = data;
    }
}
