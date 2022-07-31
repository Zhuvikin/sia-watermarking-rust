extern crate console_error_panic_hook;

use image::{DynamicImage, ImageBuffer, RgbImage};
use image::imageops::FilterType;

use wasm_bindgen::prelude::*;

use watermarking;

#[wasm_bindgen]
pub fn create_image(width: u32, height: u32) -> u32 {
    console_error_panic_hook::set_once();

    let img: RgbImage = ImageBuffer::new(width, height);

    let watermarked = watermarking::watermark(DynamicImage::from(img));

    let resized = watermarked.resize(width / 2, width / 2, FilterType::Nearest);

    return resized.width() + resized.width() + 1;
}

#[test]
fn create_image_test() {
    let image_width = 256;
    let image_height = 256;
    assert_eq!(image_width / 2 + image_height / 2 + 1, create_image(image_width, image_height));
}