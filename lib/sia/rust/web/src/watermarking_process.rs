use crate::ImageModel;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct WatermarkingProcess {
    // Source image
    pub(crate) source_format: String,
    pub(crate) source: ImageModel,

    // Output image
    pub(crate) watermarked: ImageModel,
}

#[wasm_bindgen]
impl WatermarkingProcess {
    #[wasm_bindgen(constructor)]
    pub fn new(
        input_format: String,
        source: ImageModel,
        watermarked: ImageModel,
    ) -> WatermarkingProcess {
        WatermarkingProcess {
            source_format: input_format,
            source,
            watermarked,
        }
    }

    pub fn get_source_format(&self) -> String {
        self.source_format.clone()
    }

    pub fn set_source_format(&mut self, input_format: String) {
        self.source_format = input_format;
    }

    pub fn get_source(&self) -> ImageModel {
        self.source.clone()
    }

    pub fn set_source(&mut self, source: ImageModel) {
        self.source = source;
    }

    pub fn get_watermarked(&self) -> ImageModel {
        self.watermarked.clone()
    }

    pub fn set_watermarked(&mut self, watermarked: ImageModel) {
        self.watermarked = watermarked;
    }
}
