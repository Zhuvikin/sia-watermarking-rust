use image::{ImageBuffer, RgbImage};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn create_image(width: u32, height: u32) -> u32 {
    // Construct a new RGB ImageBuffer with the specified width and height.
    let img: RgbImage = ImageBuffer::new(width, height);

    // Construct a new by repeated calls to the supplied closure.
    let mut img = ImageBuffer::from_fn(width, height, |x, y| {
        if x % 2 == 0 {
            image::Luma([0u8])
        } else {
            image::Luma([255u8])
        }
    });

    // Obtain the image's width and height.
    let (result_width, result_height) = img.dimensions();

    return result_width + result_height;
}

#[test]
fn add_test() {
    assert_eq!(512, createImage(256, 256));
}