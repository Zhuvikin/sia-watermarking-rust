use assert_approx_eq::assert_approx_eq;
use ndarray::Array2;

pub const DELTA: f64 = 1e-5;

pub fn slice2d_as_nd_array(x1: Vec<Vec<f64>>) -> Array2<f64> {
    let width = x1[0].len();
    let height = x1.len();

    let x: Vec<f64> = x1.into_iter().flatten().collect();
    let result = Array2::from_shape_vec([height, width], x).unwrap();
    result
}

pub fn assert_approximately_equals_2d(expected: &Array2<f64>, actual: &Array2<f64>) {
    for (i, row) in actual.outer_iter().enumerate() {
        for (j, element) in row.iter().enumerate() {
            let expected_element = expected[[i, j]];
            let actual_element = *element;
            assert_approx_eq!(expected_element, actual_element, DELTA);
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
