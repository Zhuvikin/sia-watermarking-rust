use crate::wavelet::image_dwt_forward;
use dwt::{transform, Operation};
use image::GrayImage;
use imageproc::contrast::equalize_histogram;

pub fn process_channel(channel: GrayImage) -> GrayImage {
    let equalized = equalize_histogram(&channel);

    image_dwt_forward(equalized, 1)
}
