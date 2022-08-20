use image::{GenericImage, GenericImageView, GrayImage, ImageBuffer, Luma};
use imageproc::contrast::equalize_histogram;
use ndarray::Array2;

use embedding::{embed, extract};
use feature::calculate_features;
use feature::feature_types::FeaturesType;
use signature::{sign, verify};
use three_bit_quantization::{three_bit_dequantization, three_bit_quantization};
use utils::bytes::vec_i64_to_bytes;

use crate::watermark::{deserialize, Watermark};

const PRIVATE_KEY: &str = "e1ec991518bead7d94b60bf64469b98d866315b3676d4fa5b4e7c36dc4951d60";
const PUBLIC_KEY: &str = "a7fd235002f60fad1a7077d74cfd62dd8922963ebc713c385ad37596fba5a7d6";

pub fn watermark_channel(
    channel: &GrayImage,
    embedding_depth: f64,
    features_quantization_step: f64,
    features_amount: usize,
    dwt_levels: usize,
) -> GrayImage {
    let raw_data = Vec::from(channel.as_raw().clone());
    let (width, height) = channel.dimensions();
    let shape = [width as usize, height as usize];
    let pixels_array = Array2::from_shape_vec(shape, raw_data).unwrap();

    // 1. Calculate channel features
    let equalized_image = equalize_histogram(channel);
    let equalized_image_array =
        Array2::from_shape_vec(shape, equalized_image.as_raw().clone()).unwrap();
    let features = calculate_features(
        equalized_image_array,
        FeaturesType::MomentsZernike,
        features_amount,
    );
    println!("features: {:?}", features);

    let (quantized_features, perturbation_vector) =
        three_bit_quantization(&features, features_quantization_step);

    println!("quantized_features: {:?}", quantized_features);
    println!("perturbation_vector: {:?}", perturbation_vector);

    let signature = sign(
        &vec_i64_to_bytes(&quantized_features),
        PRIVATE_KEY,
        PUBLIC_KEY,
    );
    println!("signature: {:?}", signature);

    let watermark = Watermark {
        signature,
        perturbation_vector,
        text: "Hello world!".to_string(),
    };
    println!("watermark: {:?}", watermark);

    let watermark_data = watermark.serialize();
    println!("watermark data: {:?}", watermark_data);

    // 2. Embed watermark data
    let watermarked = embed(pixels_array, watermark_data, embedding_depth, dwt_levels);
    let watermarked_raw = watermarked.into_raw_vec();
    GrayImage::from_raw(width, height, watermarked_raw).unwrap()
}

pub fn authenticate_channel(
    channel: &GrayImage,
    embedding_depth: f64,
    features_quantization_step: f64,
    features_amount: usize,
    dwt_levels: usize,
) -> bool {
    let raw_data = Vec::from(channel.as_raw().clone());
    let (width, height) = channel.dimensions();
    let shape = [width as usize, height as usize];
    let pixels_array = Array2::from_shape_vec(shape, raw_data).unwrap();

    // 1. Calculate channel features
    let equalized_image = equalize_histogram(channel);
    let equalized_image_array =
        Array2::from_shape_vec(shape, equalized_image.as_raw().clone()).unwrap();
    let features = calculate_features(
        equalized_image_array,
        FeaturesType::MomentsZernike,
        features_amount,
    );
    println!("features: {:?}", features);

    // 2. Extract watermark data
    let extracted_data = extract(pixels_array, embedding_depth, dwt_levels);
    println!("extracted watermark data: {:?}", extracted_data);

    let watermark = deserialize(&extracted_data);
    println!("deserialized watermark data: {:?}", watermark);

    let quantized_features = three_bit_dequantization(
        &features,
        &watermark.perturbation_vector,
        features_quantization_step,
    );
    println!("restored quantized features: {:?}", quantized_features);

    let quantized_features_bytes = vec_i64_to_bytes(&quantized_features);
    println!(
        "restored quantized features bytes: {:?}",
        quantized_features_bytes
    );

    verify(&quantized_features_bytes, &watermark.signature, PUBLIC_KEY)
}

#[cfg(test)]
mod tests {
    use image::imageops::colorops::grayscale;

    use super::*;

    #[test]
    fn channel_watermarking_test() {
        let depth = 30.0;
        let delta = 1.0;
        let features_amount = 11;

        // Embed watermark
        let original = read_gray_image("tests/peppers.png");
        let watermarked = watermark_channel(&original, depth, delta, features_amount, 3);
        watermarked.save("tests/peppers_watermarked.png").unwrap();

        // Authenticate
        let read_watermarked = read_gray_image("tests/peppers_watermarked.png");
        let is_authentic =
            authenticate_channel(&read_watermarked, depth, delta, features_amount, 3);
        assert!(is_authentic)
    }

    fn read_gray_image(path: &str) -> ImageBuffer<Luma<u8>, Vec<u8>> {
        let dynamic_image = image::open(path).unwrap();
        let (width, height) = dynamic_image.dimensions();

        let buffer = grayscale(&dynamic_image);
        GrayImage::from(buffer)
    }
}
