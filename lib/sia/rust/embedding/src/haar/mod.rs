use dwt::{transform, Operation};
use ndarray::{Array, Array2, Axis, Ix};
use utils::{
    assert_approximately_equals_1d, assert_approximately_equals_2d, vector_2d_as_nd_array,
};

pub fn dwt_forward_1d(input: Vec<f64>, level: usize) -> Vec<f64> {
    dwt_1d(input, level, false)
}

pub fn dwt_backward_1d(input: Vec<f64>, level: usize) -> Vec<f64> {
    dwt_1d(input, level, true)
}

fn dwt_1d(input: Vec<f64>, level: usize, is_backward: bool) -> Vec<f64> {
    let mut result = input.clone();
    transform(
        &mut result,
        if is_backward {
            Operation::Inverse
        } else {
            Operation::Forward
        },
        &dwt::wavelet::Haar::new(),
        level,
    );
    result
}

pub fn dwt_2d(mut input: Array2<f64>, level: usize, is_backward: bool) -> Array2<f64> {
    let mut iteration = if is_backward { level - 1 } else { 0 };
    loop {
        let (source_width, source_height) = input.dim();

        let width = source_width as i32 / 2_i32.pow(iteration as u32);
        let height = source_height as i32 / 2_i32.pow(iteration as u32);

        let mut matrix = input
            .view_mut()
            .split_at(Axis(0), width as Ix)
            .0
            .split_at(Axis(1), height as Ix)
            .0;

        // process rows
        for row in matrix.rows_mut() {
            let row_vector = row.to_vec();
            let row_dwt = dwt_1d(row_vector, 1, is_backward);
            let array = Array::from(row_dwt);
            array.assign_to(row);
        }

        // process columns
        for column in matrix.columns_mut() {
            let column_vector = column.to_vec();
            let column_dwt = dwt_1d(column_vector, 1, is_backward);
            let array = Array::from(column_dwt);
            array.assign_to(column);
        }

        if is_backward {
            if iteration == 0 {
                break;
            }
            iteration = iteration - 1;
        } else {
            if iteration == level - 1 {
                break;
            }
            iteration = iteration + 1;
        }
    }
    input
}

#[test]
fn dwt_2d_test_level_1() {
    let source = vector_2d_as_nd_array(vec![
        vec![1.0, 2.0, 3.0, 4.0],
        vec![5.0, 6.0, 7.0, 8.0],
        vec![1.0, 2.0, 3.0, 4.0],
        vec![5.0, 6.0, 7.0, 8.0],
    ]);

    let expected = vector_2d_as_nd_array(vec![
        vec![7.0, 11.0, -1.0, -1.0],
        vec![7.0, 11.0, -1.0, -1.0],
        vec![-4.0, -4.0, 0.0, 0.0],
        vec![-4.0, -4.0, 0.0, 0.0],
    ]);

    let forward_dwt = dwt_2d(source.clone(), 1, false);
    assert_approximately_equals_2d(&expected, &forward_dwt);

    let backward_dwt = dwt_2d(forward_dwt.clone(), 1, true);
    assert_approximately_equals_2d(&source, &backward_dwt);
}

#[test]
fn dwt_2d_test_level_2() {
    let source = vector_2d_as_nd_array(vec![
        vec![1.0, 2.0, 3.0, 4.0],
        vec![5.0, 6.0, 7.0, 8.0],
        vec![1.0, 2.0, 3.0, 4.0],
        vec![5.0, 6.0, 7.0, 8.0],
    ]);

    let expected = vector_2d_as_nd_array(vec![
        vec![18.0, -4.0, -1.0, -1.0],
        vec![0.0, 0.0, -1.0, -1.0],
        vec![-4.0, -4.0, 0.0, 0.0],
        vec![-4.0, -4.0, 0.0, 0.0],
    ]);

    let forward_dwt = dwt_2d(source.clone(), 2, false);
    assert_approximately_equals_2d(&expected, &forward_dwt);

    let backward_dwt = dwt_2d(forward_dwt.clone(), 2, true);
    assert_approximately_equals_2d(&source, &backward_dwt);
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

    let forward_dwt = dwt_forward_1d(source.clone(), 1);
    assert_approximately_equals_1d(&expected, &forward_dwt);

    let backward_dwt = dwt_backward_1d(forward_dwt, 1);
    assert_approximately_equals_1d(&source, &backward_dwt);
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

    let forward_dwt = dwt_forward_1d(source.clone(), 2);
    assert_approximately_equals_1d(&expected, &forward_dwt);

    let backward_dwt = dwt_backward_1d(forward_dwt, 2);
    assert_approximately_equals_1d(&source, &backward_dwt);
}
