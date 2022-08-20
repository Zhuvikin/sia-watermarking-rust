pub mod constants;
pub mod format;
pub mod math;
pub mod pseudo_random;

use assert_approx_eq::assert_approx_eq;
use ndarray::Array2;
use num::Complex;

pub const DELTA: f64 = 1e-4;

pub fn assert_approximately_equal(expected: f64, actual: f64) {
    assert_approx_eq!(expected, actual, DELTA);
}

pub fn assert_approximately_equal_complex(expected: Complex<f64>, actual: Complex<f64>) {
    assert_approx_eq!(expected.re, actual.re, DELTA);
    assert_approx_eq!(expected.im, actual.im, DELTA);
}

pub fn vector_2d_as_nd_array<T>(x1: Vec<Vec<T>>) -> Array2<T> {
    let width = x1[0].len();
    let height = x1.len();

    let x: Vec<T> = x1.into_iter().flatten().collect();
    let result = Array2::from_shape_vec([height, width], x).unwrap();
    result
}

pub fn assert_approximately_equals_2d(expected: &Array2<f64>, actual: &Array2<f64>) {
    for (i, row) in actual.outer_iter().enumerate() {
        for (j, element) in row.iter().enumerate() {
            let expected_element = expected[[i, j]];
            let actual_element = *element as f64;
            assert_approx_eq!(expected_element, actual_element, DELTA);
        }
    }
}

pub fn assert_approximately_equals_2d_complex(
    expected: &Array2<Complex<f64>>,
    actual: &Array2<Complex<f64>>,
) {
    for (i, row) in actual.outer_iter().enumerate() {
        for (j, element) in row.iter().enumerate() {
            let expected_element = expected[[i, j]];
            let actual_element = *element;
            assert_approximately_equal_complex(expected_element, actual_element);
        }
    }
}

pub fn assert_approximately_equals_1d(expected: &Vec<f64>, actual: &Vec<f64>) {
    for (i, element) in actual.iter().enumerate() {
        let expected_element = expected[i];
        let actual_element = *element;
        assert_approx_eq!(expected_element, actual_element, DELTA);
    }
}

pub fn assert_approximately_equal_tuple(expected: (f64, f64), actual: (f64, f64)) {
    let (left_expected, right_expected) = expected;
    let (left_actual, right_actual) = actual;
    assert_approx_eq!(left_expected, left_actual, DELTA);
    assert_approx_eq!(right_expected, right_actual, DELTA);
}
