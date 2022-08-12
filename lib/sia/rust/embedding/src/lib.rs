extern crate ndarray;

use ndarray::{Array2, Axis, Ix};

use crate::haar::dwt_2d;
use crate::utils::constants::get_test_matrix;
use crate::utils::{assert_approximately_equals_2d, slice2d_as_nd_array};

mod haar;
mod io;
mod utils;

static DWT_LEVELS: usize = 3;

pub fn embed(image_matrix: Array2<u8>, bytes: Vec<u8>, depth: f64) -> Array2<u8> {
    let (width, height) = image_matrix.dim();

    // 1. perform forward Haar 2D 3-level DWT
    let input_float_matrix = image_matrix.mapv(|elem| elem as f64);
    let mut dwt_coefficients = dwt_2d(input_float_matrix, DWT_LEVELS, false);

    // 2. embed to LL3 domain
    let ll3_width = width / ((2 as i32).pow(DWT_LEVELS as u32) as usize);
    let ll3_height = height / ((2 as i32).pow(DWT_LEVELS as u32) as usize);

    let mut ll3 = dwt_coefficients
        .view_mut()
        .split_at(Axis(0), ll3_width as Ix)
        .0
        .split_at(Axis(1), ll3_height as Ix)
        .0;

    io::write(&mut ll3, bytes, depth);

    // 3. perform backward Haar 2D 3-level DWT
    let restored = dwt_2d(dwt_coefficients, DWT_LEVELS, true);
    restored.mapv(|elem| elem.round() as u8)
}

pub fn extract(image_matrix: Array2<u8>, depth: f64) -> Vec<u8> {
    let (width, height) = image_matrix.dim();

    // 1. perform forward Haar 2D 3-level DWT
    let input_float_matrix = image_matrix.mapv(|elem| elem as f64);
    let mut dwt_coefficients = dwt_2d(input_float_matrix, DWT_LEVELS, false);

    // 2. embed to LL3 domain
    let ll3_width = width / ((2 as i32).pow(DWT_LEVELS as u32) as usize);
    let ll3_height = height / ((2 as i32).pow(DWT_LEVELS as u32) as usize);

    let ll3 = dwt_coefficients
        .view_mut()
        .split_at(Axis(0), ll3_width as Ix)
        .0
        .split_at(Axis(1), ll3_height as Ix)
        .0;

    io::read(&ll3, depth)
}

#[test]
fn embed_test() {
    let source = slice2d_as_nd_array(get_test_matrix());

    let forward_dwt_1l = dwt_2d(source.clone(), 3, false);
    let backward_dwt = dwt_2d(forward_dwt_1l.clone(), 3, true);
    assert_approximately_equals_2d(&source, &backward_dwt);

    let bits = vec![7];
    let depth = 100 as f64;
    let embedded = embed(source.mapv(|elem| elem.round() as u8), bits.clone(), depth);

    let extracted_bits = extract(embedded, depth);
    assert_eq!(bits, extracted_bits);
}
