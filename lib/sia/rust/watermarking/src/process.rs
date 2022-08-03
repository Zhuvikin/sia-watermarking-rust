use embedding::embed;
use image::GrayImage;
use imageproc::contrast::equalize_histogram;
use ndarray::Array2;

pub fn process_channel(channel: GrayImage) -> GrayImage {
    let raw_data = Vec::from(channel.as_raw().clone());
    let (width, height) = channel.dimensions();
    let bits_array = Array2::from_shape_vec([width as usize, height as usize], raw_data).unwrap();

    // 1. Calculate watermark data
    let equalized = equalize_histogram(&channel);
    //let watermark_data = vec![1, 0, 1, 0, 1, 1, 1];

    // 2. Embed watermark data
    //let watermarked = embed(bits_array, watermark_data, 100.0);
    //let watermarked_raw = watermarked.into_raw_vec();
    //GrayImage::from_raw(width, height, watermarked_raw).unwrap()
    equalized
}
