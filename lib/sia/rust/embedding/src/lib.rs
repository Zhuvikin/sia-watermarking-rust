extern crate ndarray;

use ndarray::{Array2, Axis, Ix};

use crate::haar::dwt_2d;
use crate::io::data::{deserialize, Data};

mod haar;
mod io;

fn get_domain_capacity(domain_width: usize, domain_height: usize) -> usize {
    let capacity = io::steganography::get_capacity(domain_width, domain_height);
    println!("domain capacity: {:?} bytes", capacity);

    let encode = encoding::get_code_for_encode(capacity);
    if encode.is_none() {
        panic!("minimum allowed embedding capacity is 16 bytes");
    }
    capacity
}

pub fn embed(
    image_matrix: Array2<u8>,
    bytes: Vec<u8>,
    depth: f64,
    dwt_levels: usize,
) -> Array2<u8> {
    let (width, height) = image_matrix.dim();

    // 1. perform forward Haar 2D multi-level DWT
    let input_float_matrix = image_matrix.mapv(|elem| elem as f64);
    let mut dwt_coefficients = dwt_2d(input_float_matrix, dwt_levels, false);

    // 2. embed to LL3 domain
    let ll3_width = width / ((2 as i32).pow(dwt_levels as u32) as usize);
    let ll3_height = height / ((2 as i32).pow(dwt_levels as u32) as usize);

    let mut ll3 = dwt_coefficients
        .view_mut()
        .split_at(Axis(0), ll3_width as Ix)
        .0
        .split_at(Axis(1), ll3_height as Ix)
        .0;

    let domain_capacity = get_domain_capacity(ll3_width, ll3_height);
    let data = Data { bytes };
    let serialized_data = data.serialize();

    let encoded = encoding::encode(serialized_data, domain_capacity);
    io::write(&mut ll3, encoded, depth);

    // 3. perform backward Haar 2D 3-level DWT
    let restored = dwt_2d(dwt_coefficients, dwt_levels, true);
    restored.mapv(|elem| elem.round() as u8)
}

pub fn extract(image_matrix: Array2<u8>, depth: f64, dwt_levels: usize) -> Vec<u8> {
    let (width, height) = image_matrix.dim();

    // 1. perform forward Haar 2D multi-level DWT
    let input_float_matrix = image_matrix.mapv(|elem| elem as f64);
    let mut dwt_coefficients = dwt_2d(input_float_matrix, dwt_levels, false);

    // 2. embed to LL3 domain
    let ll3_width = width / ((2 as i32).pow(dwt_levels as u32) as usize);
    let ll3_height = height / ((2 as i32).pow(dwt_levels as u32) as usize);

    let ll3 = dwt_coefficients
        .view_mut()
        .split_at(Axis(0), ll3_width as Ix)
        .0
        .split_at(Axis(1), ll3_height as Ix)
        .0;

    let extracted = io::read(&ll3, depth);

    let domain_capacity = get_domain_capacity(ll3_width, ll3_height);
    let decoded = encoding::decode(extracted, domain_capacity);

    let data = deserialize(&decoded);
    data.bytes
}

#[cfg(test)]
mod tests {
    use utils::constants::get_test_32_32_matrix;

    use utils::{assert_approximately_equals_2d, vector_2d_as_nd_array};

    use super::*;

    #[test]
    fn embed_test() {
        let source = vector_2d_as_nd_array(get_test_32_32_matrix());

        let forward_dwt = dwt_2d(source.clone(), 1, false);
        let backward_dwt = dwt_2d(forward_dwt.clone(), 1, true);
        assert_approximately_equals_2d(&source, &backward_dwt);

        let bytes = vec![113, 117];
        let depth = 100 as f64;
        let embedded = embed(
            source.mapv(|elem| elem.round() as u8),
            bytes.clone(),
            depth,
            1,
        );

        let extracted_bits = extract(embedded, depth, 1);
        assert_eq!(bytes, extracted_bits);
    }
}
