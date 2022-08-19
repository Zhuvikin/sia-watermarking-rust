use embedding::{embed, extract};
use encoding::{decode, encode};
use feature::calculate_features;
use feature::feature_types::FeaturesType;
use image::{GenericImage, GenericImageView, GrayImage, ImageBuffer, Luma};
use imageproc::contrast::equalize_histogram;
use ndarray::Array2;
use three_bit_quantization::three_bit_quantization;

pub fn watermark_channel(
    channel: &GrayImage,
    embedding_depth: f64,
    features_quantization_step: f64,
    features_amount: usize,
) -> GrayImage {
    let raw_data = Vec::from(channel.as_raw().clone());
    let (width, height) = channel.dimensions();
    let shape = [width as usize, height as usize];
    let pixels_array = Array2::from_shape_vec(shape, raw_data).unwrap();

    // 1. Calculate watermark data
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

    let watermark_data: Vec<u8> = vec![123, 43, 65, 0, 255, 1, 7];
    let encoded_data = encode(watermark_data);

    // 2. Embed watermark data
    let watermarked = embed(pixels_array, encoded_data, embedding_depth);
    let watermarked_raw = watermarked.into_raw_vec();
    GrayImage::from_raw(width, height, watermarked_raw).unwrap()
}

pub fn authenticate_channel(
    channel: &GrayImage,
    embedding_depth: f64,
    delta: f64,
    features_amount: usize,
) -> bool {
    let raw_data = Vec::from(channel.as_raw().clone());
    let (width, height) = channel.dimensions();
    let pixels_array = Array2::from_shape_vec([width as usize, height as usize], raw_data).unwrap();

    // 1. Calculate watermark data
    let equalized = equalize_histogram(channel);
    let expected_watermark_data: Vec<u8> = vec![123, 43, 65, 0, 255, 1, 7];

    // 2. Embed watermark data
    let extracted_data = extract(pixels_array, embedding_depth);
    let decoded_data = decode(extracted_data);

    decoded_data == expected_watermark_data
}

#[test]
fn channel_watermarking_test() {
    let depth = 30.0;
    let delta = 1.0;
    let features_amount = 11;

    // Embed watermark
    let original = read_gray_image("tests/aerial.jpg");
    let watermarked = watermark_channel(&original, depth, delta, features_amount);
    watermarked.save("tests/aerial_watermarked.jpg").unwrap();

    // Authenticate
    let read_watermarked = read_gray_image("tests/aerial_watermarked.jpg");
    let is_authentic = authenticate_channel(&read_watermarked, depth, delta, features_amount);
    assert!(is_authentic)
}

fn read_gray_image(path: &str) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let dynamic_image = image::open(path).unwrap();
    let (width, height) = dynamic_image.dimensions();
    let mut gray = GrayImage::new(width, height);

    let image1 = dynamic_image.grayscale();
    let gray2 = image1.as_luma8().unwrap();
    gray.copy_from(gray2, 0, 0);
    gray
}
