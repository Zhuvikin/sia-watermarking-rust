mod wavelet;

use image::GrayImage;
use imageproc::contrast::equalize_histogram;

pub fn process_channel(channel: GrayImage) -> GrayImage {
    let equalized = equalize_histogram(&channel);

    wavelet::image_dwt_forward(equalized, 1)
}
