use puruspe::gamma;

pub fn factorial(num: f64) -> f64 {
    if num.fract() != 0.0 {
        return gamma(1. + num);
    }
    match num.round() as i64 {
        0 | 1 => 1.,
        _ => factorial(num - 1.) * num,
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_approximately_equal;
    use crate::math::factorial;

    #[test]
    fn calculate_zernike_matrix_test() {
        assert_approximately_equal(-4.32685, factorial(-1.3));
        assert_eq!(1., factorial(0.));
        assert_approximately_equal(0.886227, factorial(0.5));
        assert_eq!(1., factorial(1.));
        assert_approximately_equal(1.32934, factorial(1.5));
        assert_eq!(2., factorial(2.));
        assert_eq!(6., factorial(3.));
        assert_eq!(24., factorial(4.));
    }
}
