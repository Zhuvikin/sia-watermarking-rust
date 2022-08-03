mod haar;

use crate::haar::constants::get_test_matrix;
use crate::haar::dwt_2d;
use crate::haar::utils::{assert_approximately_equals_2d, slice2d_as_nd_array};
use ndarray::{Array2, ArrayViewMut, AssignElem, Axis, Ix, Ix2};

static EOF: &'static [u8] = &[1, 1, 1, 1, 1, 1, 1, 1];
static DWT_LEVELS: usize = 3;

pub fn embed(image_matrix: Array2<u8>, bits: Vec<u8>, depth: f64) -> Array2<u8> {
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

    embed_to_domain(ll3, bits, depth);

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

    let bits = extract_from_domain(ll3, depth);
    bits
}

fn embed_to_domain(mut domain: ArrayViewMut<f64, Ix2>, bits: Vec<u8>, depth: f64) {
    let data_length = bits.len();
    let (width, height) = domain.dim();
    let capacity = available_capacity(width as usize, height as usize);
    let data_with_eof_length = data_length + EOF.len();
    if data_with_eof_length > capacity {
        panic!(
            "length of data + EOF to be embedded ({:?} bits) is greater than available capacity ({:?} bits)",
            data_with_eof_length, capacity
        );
    }

    let data_with_eof_padded;
    if data_with_eof_length == capacity {
        data_with_eof_padded = bits.clone();
    } else {
        data_with_eof_padded =
            [&bits[..], &EOF, &vec![0; capacity - data_with_eof_length]].concat();
    }

    for (i, coefficient) in domain.iter_mut().enumerate() {
        let coefficient_with_bit =
            embed_bit(coefficient.to_owned(), data_with_eof_padded[i], depth);
        coefficient.assign_elem(coefficient_with_bit);
    }
}

fn extract_from_domain(domain: ArrayViewMut<f64, Ix2>, depth: f64) -> Vec<u8> {
    let mut extracted = vec![];
    let mut last_byte = vec![0; EOF.len()];
    for coefficient in domain.iter() {
        let bit = extract_bit(*coefficient, depth);
        extracted.push(bit);
        last_byte.remove(0);
        last_byte.push(bit);
        if last_byte == EOF {
            for _ in 1..EOF.len() {
                extracted.pop();
            }
            println!("{:?} bits were extracted", extracted.len());
            return extracted;
        }
    }
    panic!("failed to find EOF")
}

fn embed_bit(coefficient: f64, bit: u8, depth: f64) -> f64 {
    let mut reminder = 0.25;
    if bit == 0 {
        reminder = -0.25;
    }
    depth * ((coefficient / depth).round() as f64 + reminder)
}

fn extract_bit(coefficient: f64, depth: f64) -> u8 {
    if coefficient - depth * (coefficient / depth).round() >= 0 as f64 {
        1
    } else {
        0
    }
}

fn available_capacity(width: usize, height: usize) -> usize {
    width * height
}

#[test]
fn embed_test() {
    let source = slice2d_as_nd_array(get_test_matrix());

    let forward_dwt_1l = dwt_2d(source.clone(), 3, false);
    let backward_dwt = dwt_2d(forward_dwt_1l.clone(), 3, true);
    assert_approximately_equals_2d(&source, &backward_dwt);

    let bits = vec![0, 1, 0, 1];
    let depth = 100 as f64;
    let embedded = embed(source.mapv(|elem| elem.round() as u8), bits.clone(), depth);

    let extracted_bits = extract(embedded, depth);
    assert_eq!(bits, extracted_bits);
}

#[test]
fn embed_to_domain_test() {
    let depth = 100 as f64;

    let mut ll3 = slice2d_as_nd_array(vec![
        vec![873.5000, 906.5000, 1104.875, 944.6250],
        vec![976.7500, 925.5000, 1032.125, 1078.250],
        vec![1028.125, 1057.000, 1051.875, 998.6250],
        vec![985.1250, 978.6250, 991.1250, 1023.750],
    ]);

    let bits = vec![0, 1, 0, 1];
    embed_to_domain(ll3.view_mut(), bits.clone(), depth);

    let extracted_bits = extract_from_domain(ll3.view_mut(), depth);
    assert_eq!(bits, extracted_bits);
}

#[test]
fn bits_embedding_test() {
    let coefficients = vec![1023.0, 1.0, -123.0, 32424.2, -20.2, 1435.01, -12334.0];
    for coefficient in coefficients {
        let bits = vec![0, 1, 0, 1];
        let depth = 100 as f64;

        let embedded: Vec<f64> = bits
            .iter()
            .map(|&b| embed_bit(coefficient, b, depth))
            .collect();
        println!("embedded: {:?}", embedded);

        let extracted: Vec<u8> = embedded.iter().map(|&c| extract_bit(c, depth)).collect();
        println!("extracted: {:?}", extracted);

        assert_eq!(bits, extracted);
    }
}
