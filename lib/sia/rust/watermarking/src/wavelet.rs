use assert_approx_eq::assert_approx_eq;
use dwt::{transform, Operation};
use image::GrayImage;

const DELTA: f64 = 1e-5;

pub fn image_dwt_forward(image: GrayImage, level: usize) -> GrayImage {
    let data = dwt_forward_1d(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0], level);

    println!("{:?}", data);

    image
}

pub fn dwt_forward_1d(input: Vec<f64>, level: usize) -> Vec<f64> {
    dwt_1d(input, level, true)
}

pub fn dwt_backward_1d(input: Vec<f64>, level: usize) -> Vec<f64> {
    dwt_1d(input, level, false)
}

fn dwt_1d(input: Vec<f64>, level: usize, x: bool) -> Vec<f64> {
    let mut result = input.clone();
    transform(
        &mut result,
        if x {
            Operation::Forward
        } else {
            Operation::Inverse
        },
        &dwt::wavelet::Haar::new(),
        level,
    );
    result
}

pub fn dwt_forward_2d(input: Vec<f64>, width: usize, height: usize, level: usize) -> Vec<f64> {
    let mut transformed_rows = vec![];
    for i in 0..height {
        let mut row = vec![0.0; width];
        row.copy_from_slice(&input[i * height..i * height + width]);
        let row_dwt = dwt_1d(row, level, true);
        transformed_rows = [&transformed_rows[..], &row_dwt[..]].concat();
    }

    let mut transformed_columns = vec![];
    for i in 0..width {
        let column = transformed_rows
            .iter()
            .skip(i)
            .step_by(height)
            .copied()
            .collect();
        let column_dwt = dwt_1d(column, level, true);
        transformed_columns = [&transformed_columns[..], &column_dwt[..]].concat();
    }

    let mut result: Vec<f64> = vec![];
    for i in 0..height {
        let row: Vec<f64> = transformed_columns
            .iter()
            .skip(i)
            .step_by(width)
            .copied()
            .collect();
        result = [&result[..], &row[..]].concat();
    }
    result
}

#[test]
fn dwt_2d_test_level_1() {
    let source = vec![
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0,
    ];

    let expected = vec![
        7.0, 11.0, -1.0, -1.0, 7.0, 11.0, -1.0, -1.0, -4.0, -4.0, 0.0, 0.0, -4.0, -4.0, 0.0, 0.0,
    ];

    // test forward 2D transform
    let actual = dwt_forward_2d(source.clone(), 4, 4, 1);
    for (i, element) in actual.iter().enumerate() {
        let expected_element = expected[i];
        let actual_element = *element;
        assert_approx_eq!(expected_element, actual_element, DELTA);
    }
}

#[test]
fn dwt_1d_test_level_1() {
    let source = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let frac1sqrt2 = std::f64::consts::FRAC_1_SQRT_2;

    let expected = vec![
        2.12132,
        4.94975,
        7.77817,
        10.6066,
        -frac1sqrt2,
        -frac1sqrt2,
        -frac1sqrt2,
        -frac1sqrt2,
    ];

    // test forward transform
    let mut actual = dwt_forward_1d(source.clone(), 1);
    for (i, element) in actual.iter().enumerate() {
        let expected_element = expected[i];
        let actual_element = *element;
        assert_approx_eq!(expected_element, actual_element, DELTA);
    }

    // test backward transform
    actual = dwt_backward_1d(actual, 1);
    for (i, element) in actual.iter().enumerate() {
        let expected_element = source[i];
        let actual_element = *element;
        assert_approx_eq!(expected_element, actual_element, DELTA);
    }
}

#[test]
fn dwt_1d_test_level_2() {
    let source = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let frac1sqrt2 = std::f64::consts::FRAC_1_SQRT_2;

    let expected = vec![
        5.,
        13.,
        -2.,
        -2.,
        -frac1sqrt2,
        -frac1sqrt2,
        -frac1sqrt2,
        -frac1sqrt2,
    ];

    let mut actual = dwt_forward_1d(source.clone(), 2);
    for (i, element) in actual.iter().enumerate() {
        let expected_element = expected[i];
        let actual_element = *element;
        assert_approx_eq!(expected_element, actual_element, DELTA);
    }

    actual = dwt_backward_1d(actual, 2);
    for (i, element) in actual.iter().enumerate() {
        let expected_element = source[i];
        let actual_element = *element;
        assert_approx_eq!(expected_element, actual_element, DELTA);
    }
}
