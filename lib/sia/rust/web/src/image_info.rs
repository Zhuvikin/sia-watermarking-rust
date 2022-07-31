use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, PartialEq)]
pub struct ImageInfo {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) format: String,
}

#[wasm_bindgen]
impl ImageInfo {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, format: String) -> ImageInfo {
        ImageInfo {
            width,
            height,
            format,
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
}
