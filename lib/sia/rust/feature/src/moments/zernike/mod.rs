use crate::moments::zernike::constants::I;
use ndarray::{Array2, Ix2};
use num::complex::{Complex64, ComplexFloat};
use num::Complex;
use std::f64::consts::{FRAC_1_SQRT_2, PI};
use utils::math::factorial;

mod constants;

pub fn calculate_zernike_image_moment(
    order: usize,
    index: usize,
    image: &Array2<u8>,
) -> Complex<f64> {
    let (width, height) = image.dim();

    let zernike_matrix_vector = get_zernike_matrix(order, index as isize, width).into_raw_vec();

    let integral =
        image
            .iter()
            .enumerate()
            .fold(Complex::new(0., 0.), |accumulator, (i, &next)| {
                let next_pixel = next as f64;
                let zernike_multiplier = zernike_matrix_vector.get(i).unwrap();

                return accumulator + zernike_multiplier * next_pixel;
            });

    let multiplier = (order as f64 + 1.) / PI;
    let pixels_amount = (width * height) as f64;
    multiplier * integral / pixels_amount
}

pub fn get_order_by_features_amount(amount: usize) -> usize {
    1 + 2 * (amount - 1)
}

fn f(order: usize, index: usize, s: f64, r: f64) -> f64 {
    let term: f64 = if r == 0. && order as f64 - 2. * s == 0. {
        1.
    } else {
        r.powf((order as isize - 2 * s as isize) as f64)
    };

    let numerator = factorial(order as f64 - s);
    let denominator_1 = factorial(s);
    let denominator_2 = factorial((order as f64 + index as f64) / 2. - s);
    let denominator_3 = factorial((order as f64 - index as f64) / 2. - s);

    numerator / (denominator_1 * denominator_2 * denominator_3) * term
}

fn radial(order: usize, index: usize, r: f64) -> f64 {
    let end = ((order as f64 - index as f64) / 2.).floor() as usize;
    (0..=end).fold(0.0, |accumulator, s| {
        accumulator + (-1.0f64).powf(s as f64) * f(order, index, s as f64, r)
    })
}

fn v(order: usize, index: usize, r: f64, theta: f64) -> Complex<f64> {
    radial(order, index, r) * (I * index as f64 * theta).exp()
}

fn to_polar(x: f64, y: f64, dimension: usize) -> (f64, f64) {
    let xc: f64 = FRAC_1_SQRT_2 * (1. + dimension as f64 - 2. * x) / (1. - dimension as f64);
    let yc: f64 = FRAC_1_SQRT_2 * (1. + dimension as f64 - 2. * y) / (1. - dimension as f64);
    if xc == 0. && yc == 0. {
        (0., 0.)
    } else {
        ((xc.powf(2.) + yc.powf(2.)).sqrt(), yc.atan2(xc))
    }
}

pub fn get_zernike_matrix(order: usize, index: isize, dimension: usize) -> Array2<Complex<f64>> {
    let zernike_matrix_vector = vec![Complex::new(0., 0.); dimension * dimension]
        .iter()
        .enumerate()
        .map(|(i, element)| {
            let y = (i.rem_euclid(dimension) + 1) as f64;
            let x = ((i as f64) / (dimension as f64)).floor() + 1.;
            let (radius, angle) = to_polar(x, y, dimension);

            v(order, index.abs() as usize, radius, angle).conj()
        })
        .collect();
    Array2::from_shape_vec([dimension, dimension], zernike_matrix_vector).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::{FRAC_1_PI, FRAC_PI_2, FRAC_PI_4};
    use utils::{
        assert_approximately_equal, assert_approximately_equal_complex,
        assert_approximately_equal_tuple, assert_approximately_equals_2d,
        assert_approximately_equals_2d_complex, vector_2d_as_nd_array,
    };

    #[test]
    fn f_test() {
        assert_approximately_equal(1., f(0, 0, 0., 0.));
        assert_approximately_equal(1., f(0, 0, 0., 0.1));
        assert_approximately_equal(1., f(0, 0, 0., 0.3));
        assert_approximately_equal(0., f(1, 0, 0., 0.));
        assert_approximately_equal(0., f(16, 0, 5., 0.));
        assert_approximately_equal(0., f(17, 0, 4., 0.));
        assert_approximately_equal(0.0101859, f(4, 3, 1., 0.1));
        assert_approximately_equal(0.0407437, f(4, 3, 1., 0.2));
        assert_approximately_equal(0.0916732, f(4, 3, 1., 0.3));
    }

    #[test]
    fn radial_test() {
        assert_approximately_equal(0.000232821, radial(4, 3, 0.1));
        assert_approximately_equal(0.00372514, radial(4, 3, 0.2));
        assert_approximately_equal(0.0188585, radial(4, 3, 0.3));
    }

    #[test]
    fn v_test() {
        assert_approximately_equal_complex(0.000222422 + 0.0000688033 * I, v(4, 3, 0.1, 0.1));
        assert_approximately_equal_complex(0.000192155 + 0.000131461 * I, v(4, 3, 0.1, 0.2));
        assert_approximately_equal_complex(0.000144724 + 0.000182375 * I, v(4, 3, 0.1, 0.3));

        assert_approximately_equal_complex(0.00355876 + 0.00110085 * I, v(4, 3, 0.2, 0.1));
        assert_approximately_equal_complex(0.00307449 + 0.00210337 * I, v(4, 3, 0.2, 0.2));
        assert_approximately_equal_complex(0.00231558 + 0.002918 * I, v(4, 3, 0.2, 0.3));

        assert_approximately_equal_complex(0.00231558 + 0.002918 * I, v(4, 3, 0.2, 0.3));
    }

    #[test]
    fn to_polar_test() {
        assert_approximately_equal_tuple((1., -2.35619), to_polar(1., 1., 2));
        assert_approximately_equal_tuple((1., -2.35619), to_polar(1., 1., 3));
        assert_approximately_equal_tuple((1., -2.35619), to_polar(1., 1., 8));

        assert_approximately_equal_tuple((1.0, -FRAC_PI_4), to_polar(2., 1., 2));
        assert_approximately_equal_tuple((FRAC_1_SQRT_2, -FRAC_PI_2), to_polar(2., 1., 3));
        assert_approximately_equal_tuple((0.868966, -2.19105), to_polar(2., 1., 8));

        assert_approximately_equal_tuple((1.07992, -3.01686), to_polar(-3.123, 7.48, 17));
        assert_approximately_equal_tuple((1.90942, -1.41275), to_polar(18.1, -19., 25));
        assert_approximately_equal_tuple((20.5549, -2.3343), to_polar(-18.1, -19., 3));
    }

    #[test]
    fn calculate_zernike_matrix_test() {
        assert_approximately_equals_2d_complex(
            &vector_2d_as_nd_array(constants::matrix_zernike_0_0()),
            &get_zernike_matrix(0, 0, 8),
        );

        assert_approximately_equals_2d_complex(
            &vector_2d_as_nd_array(constants::matrix_zernike_1_1()),
            &get_zernike_matrix(1, 1, 8),
        );

        assert_approximately_equals_2d_complex(
            &vector_2d_as_nd_array(constants::matrix_zernike_2_0()),
            &get_zernike_matrix(2, 0, 8),
        );

        assert_approximately_equals_2d_complex(
            &vector_2d_as_nd_array(constants::matrix_zernike_2_2()),
            &get_zernike_matrix(2, 2, 8),
        );

        assert_approximately_equals_2d_complex(
            &vector_2d_as_nd_array(constants::matrix_zernike_3_1()),
            &get_zernike_matrix(3, 1, 8),
        );

        assert_approximately_equals_2d_complex(
            &vector_2d_as_nd_array(constants::matrix_zernike_3_3()),
            &get_zernike_matrix(3, 3, 8),
        );
    }

    #[test]
    fn calculate_zernike_moment_test() {
        let image = vector_2d_as_nd_array(vec![
            vec![160, 26, 188, 252, 3, 49, 212, 206],
            vec![240, 126, 183, 47, 40, 49, 111, 168],
            vec![196, 140, 141, 191, 94, 174, 15, 149],
            vec![72, 155, 171, 156, 204, 71, 166, 2],
            vec![244, 164, 164, 65, 51, 137, 185, 123],
            vec![252, 42, 104, 254, 61, 241, 84, 133],
            vec![146, 174, 29, 145, 85, 56, 80, 130],
            vec![114, 67, 163, 80, 165, 129, 101, 42],
        ]);

        assert_approximately_equal_complex(
            40.6193 + 0. * I,
            calculate_zernike_image_moment(0, 0, &image),
        );

        assert_approximately_equal_complex(
            -1.90011 + 4.21521 * I,
            calculate_zernike_image_moment(1, 1, &image),
        );

        assert_approximately_equal_complex(
            -16.2877 + 0. * I,
            calculate_zernike_image_moment(2, 0, &image),
        );

        assert_approximately_equal_complex(
            -2.94152 + 0.893115 * I,
            calculate_zernike_image_moment(2, 2, &image),
        );

        assert_approximately_equal_complex(
            -17.2633 - 12.6124 * I,
            calculate_zernike_image_moment(17, 7, &image),
        );
    }
}
