use ndarray::Array2;
use num::complex::Complex;
use std::f64::consts::FRAC_1_SQRT_2;

pub const I: Complex<f64> = Complex::new(0., 1.);

pub fn matrix_zernike_0_0() -> Vec<Vec<Complex<f64>>> {
    vec![
        vec![
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
        ],
        vec![
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
        ],
        vec![
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
        ],
        vec![
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
        ],
        vec![
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
        ],
        vec![
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
        ],
        vec![
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
        ],
        vec![
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
            1. + 0. * I,
        ],
    ]
}

pub fn matrix_zernike_1_1() -> Vec<Vec<Complex<f64>>> {
    vec![
        vec![
            -FRAC_1_SQRT_2 + FRAC_1_SQRT_2 * I,
            -FRAC_1_SQRT_2 + 0.505076 * I,
            -FRAC_1_SQRT_2 + 0.303046 * I,
            -FRAC_1_SQRT_2 + 0.101015 * I,
            -FRAC_1_SQRT_2 + -0.101015 * I,
            -FRAC_1_SQRT_2 + -0.303046 * I,
            -FRAC_1_SQRT_2 + -0.505076 * I,
            -FRAC_1_SQRT_2 + -FRAC_1_SQRT_2 * I,
        ],
        vec![
            -0.505076 + FRAC_1_SQRT_2 * I,
            -0.505076 + 0.505076 * I,
            -0.505076 + 0.303046 * I,
            -0.505076 + 0.101015 * I,
            -0.505076 + -0.101015 * I,
            -0.505076 + -0.303046 * I,
            -0.505076 + -0.505076 * I,
            -0.505076 + -FRAC_1_SQRT_2 * I,
        ],
        vec![
            -0.303046 + FRAC_1_SQRT_2 * I,
            -0.303046 + 0.505076 * I,
            -0.303046 + 0.303046 * I,
            -0.303046 + 0.101015 * I,
            -0.303046 + -0.101015 * I,
            -0.303046 + -0.303046 * I,
            -0.303046 + -0.505076 * I,
            -0.303046 + -FRAC_1_SQRT_2 * I,
        ],
        vec![
            -0.101015 + FRAC_1_SQRT_2 * I,
            -0.101015 + 0.505076 * I,
            -0.101015 + 0.303046 * I,
            -0.101015 + 0.101015 * I,
            -0.101015 + -0.101015 * I,
            -0.101015 + -0.303046 * I,
            -0.101015 + -0.505076 * I,
            -0.101015 + -FRAC_1_SQRT_2 * I,
        ],
        vec![
            0.101015 + FRAC_1_SQRT_2 * I,
            0.101015 + 0.505076 * I,
            0.101015 + 0.303046 * I,
            0.101015 + 0.101015 * I,
            0.101015 + -0.101015 * I,
            0.101015 + -0.303046 * I,
            0.101015 + -0.505076 * I,
            0.101015 + -FRAC_1_SQRT_2 * I,
        ],
        vec![
            0.303046 + FRAC_1_SQRT_2 * I,
            0.303046 + 0.505076 * I,
            0.303046 + 0.303046 * I,
            0.303046 + 0.101015 * I,
            0.303046 + -0.101015 * I,
            0.303046 + -0.303046 * I,
            0.303046 + -0.505076 * I,
            0.303046 + -FRAC_1_SQRT_2 * I,
        ],
        vec![
            0.505076 + FRAC_1_SQRT_2 * I,
            0.505076 + 0.505076 * I,
            0.505076 + 0.303046 * I,
            0.505076 + 0.101015 * I,
            0.505076 + -0.101015 * I,
            0.505076 + -0.303046 * I,
            0.505076 + -0.505076 * I,
            0.505076 + -FRAC_1_SQRT_2 * I,
        ],
        vec![
            FRAC_1_SQRT_2 + FRAC_1_SQRT_2 * I,
            FRAC_1_SQRT_2 + 0.505076 * I,
            FRAC_1_SQRT_2 + 0.303046 * I,
            FRAC_1_SQRT_2 + 0.101015 * I,
            FRAC_1_SQRT_2 + -0.101015 * I,
            FRAC_1_SQRT_2 + -0.303046 * I,
            FRAC_1_SQRT_2 + -0.505076 * I,
            FRAC_1_SQRT_2 + -FRAC_1_SQRT_2 * I,
        ],
    ]
}

pub fn matrix_zernike_2_0() -> Vec<Vec<Complex<f64>>> {
    vec![
        vec![
            1. + 0. * I,
            0.510204 + 0. * I,
            0.183673 + 0. * I,
            0.0204082 + 0. * I,
            0.0204082 + 0. * I,
            0.183673 + 0. * I,
            0.510204 + 0. * I,
            1. + 0. * I,
        ],
        vec![
            0.510204 + 0. * I,
            0.0204082 + 0. * I,
            -0.306122 + 0. * I,
            -0.469388 + 0. * I,
            -0.469388 + 0. * I,
            -0.306122 + 0. * I,
            0.0204082 + 0. * I,
            0.510204 + 0. * I,
        ],
        vec![
            0.183673 + 0. * I,
            -0.306122 + 0. * I,
            -0.632653 + 0. * I,
            -0.795918 + 0. * I,
            -0.795918 + 0. * I,
            -0.632653 + 0. * I,
            -0.306122 + 0. * I,
            0.183673 + 0. * I,
        ],
        vec![
            0.0204082 + 0. * I,
            -0.469388 + 0. * I,
            -0.795918 + 0. * I,
            -0.959184 + 0. * I,
            -0.959184 + 0. * I,
            -0.795918 + 0. * I,
            -0.469388 + 0. * I,
            0.0204082 + 0. * I,
        ],
        vec![
            0.0204082 + 0. * I,
            -0.469388 + 0. * I,
            -0.795918 + 0. * I,
            -0.959184 + 0. * I,
            -0.959184 + 0. * I,
            -0.795918 + 0. * I,
            -0.469388 + 0. * I,
            0.0204082 + 0. * I,
        ],
        vec![
            0.183673 + 0. * I,
            -0.306122 + 0. * I,
            -0.632653 + 0. * I,
            -0.795918 + 0. * I,
            -0.795918 + 0. * I,
            -0.632653 + 0. * I,
            -0.306122 + 0. * I,
            0.183673 + 0. * I,
        ],
        vec![
            0.510204 + 0. * I,
            0.0204082 + 0. * I,
            -0.306122 + 0. * I,
            -0.469388 + 0. * I,
            -0.469388 + 0. * I,
            -0.306122 + 0. * I,
            0.0204082 + 0. * I,
            0.510204 + 0. * I,
        ],
        vec![
            1. + 0. * I,
            0.510204 + 0. * I,
            0.183673 + 0. * I,
            0.0204082 + 0. * I,
            0.0204082 + 0. * I,
            0.183673 + 0. * I,
            0.510204 + 0. * I,
            1. + 0. * I,
        ],
    ]
}

pub fn matrix_zernike_2_2() -> Vec<Vec<Complex<f64>>> {
    vec![
        vec![
            0. - 1. * I,
            0.244898 - 0.714286 * I,
            0.408163 - 0.428571 * I,
            0.489796 - 0.142857 * I,
            0.489796 + 0.142857 * I,
            0.408163 + 0.428571 * I,
            0.244898 + 0.714286 * I,
            0. + 1. * I,
        ],
        vec![
            -0.244898 - 0.714286 * I,
            0. - 0.510204 * I,
            0.163265 - 0.306122 * I,
            0.244898 - 0.102041 * I,
            0.244898 + 0.102041 * I,
            0.163265 + 0.306122 * I,
            0. + 0.510204 * I,
            -0.244898 + 0.714286 * I,
        ],
        vec![
            -0.408163 - 0.428571 * I,
            -0.163265 - 0.306122 * I,
            0. - 0.183673 * I,
            0.0816327 - 0.0612245 * I,
            0.0816327 + 0.0612245 * I,
            0. + 0.183673 * I,
            -0.163265 + 0.306122 * I,
            -0.408163 + 0.428571 * I,
        ],
        vec![
            -0.489796 - 0.142857 * I,
            -0.244898 - 0.102041 * I,
            -0.0816327 - 0.0612245 * I,
            0. - 0.0204082 * I,
            0. + 0.0204082 * I,
            -0.0816327 + 0.0612245 * I,
            -0.244898 + 0.102041 * I,
            -0.489796 + 0.142857 * I,
        ],
        vec![
            -0.489796 + 0.142857 * I,
            -0.244898 + 0.102041 * I,
            -0.0816327 + 0.0612245 * I,
            0. + 0.0204082 * I,
            0. - 0.0204082 * I,
            -0.0816327 - 0.0612245 * I,
            -0.244898 - 0.102041 * I,
            -0.489796 - 0.142857 * I,
        ],
        vec![
            -0.408163 + 0.428571 * I,
            -0.163265 + 0.306122 * I,
            0. + 0.183673 * I,
            0.0816327 + 0.0612245 * I,
            0.0816327 - 0.0612245 * I,
            0. - 0.183673 * I,
            -0.163265 - 0.306122 * I,
            -0.408163 - 0.428571 * I,
        ],
        vec![
            -0.244898 + 0.714286 * I,
            0. + 0.510204 * I,
            0.163265 + 0.306122 * I,
            0.244898 + 0.102041 * I,
            0.244898 - 0.102041 * I,
            0.163265 - 0.306122 * I,
            0. - 0.510204 * I,
            -0.244898 - 0.714286 * I,
        ],
        vec![
            0. + 1. * I,
            0.244898 + 0.714286 * I,
            0.408163 + 0.428571 * I,
            0.489796 + 0.142857 * I,
            0.489796 - 0.142857 * I,
            0.408163 - 0.428571 * I,
            0.244898 - 0.714286 * I,
            0. - 1. * I,
        ],
    ]
}

pub fn matrix_zernike_3_1() -> Vec<Vec<Complex<f64>>> {
    vec![
        vec![
            -FRAC_1_SQRT_2 + FRAC_1_SQRT_2 * I,
            -0.1876 + 0.134 * I,
            0.158738 - 0.0680307 * I,
            0.331907 - 0.0474153 * I,
            0.331907 + 0.0474153 * I,
            0.158738 + 0.0680307 * I,
            -0.1876 - 0.134 * I,
            -FRAC_1_SQRT_2 - FRAC_1_SQRT_2 * I,
        ],
        vec![
            -0.134 + 0.1876 * I,
            0.237077 - 0.237077 * I,
            0.484461 - 0.290677 * I,
            0.608153 - 0.121631 * I,
            0.608153 + 0.121631 * I,
            0.484461 + 0.290677 * I,
            0.237077 + 0.237077 * I,
            -0.134 - 0.1876 * I,
        ],
        vec![
            0.0680307 - 0.158738 * I,
            0.290677 - 0.484461 * I,
            0.439107 - 0.439107 * I,
            0.513322 - 0.171107 * I,
            0.513322 + 0.171107 * I,
            0.439107 + 0.439107 * I,
            0.290677 + 0.484461 * I,
            0.0680307 + 0.158738 * I,
        ],
        vec![
            0.0474153 - 0.331907 * I,
            0.121631 - 0.608153 * I,
            0.171107 - 0.513322 * I,
            0.195846 - 0.195846 * I,
            0.195846 + 0.195846 * I,
            0.171107 + 0.513322 * I,
            0.121631 + 0.608153 * I,
            0.0474153 + 0.331907 * I,
        ],
        vec![
            -0.0474153 - 0.331907 * I,
            -0.121631 - 0.608153 * I,
            -0.171107 - 0.513322 * I,
            -0.195846 - 0.195846 * I,
            -0.195846 + 0.195846 * I,
            -0.171107 + 0.513322 * I,
            -0.121631 + 0.608153 * I,
            -0.0474153 + 0.331907 * I,
        ],
        vec![
            -0.0680307 - 0.158738 * I,
            -0.290677 - 0.484461 * I,
            -0.439107 - 0.439107 * I,
            -0.513322 - 0.171107 * I,
            -0.513322 + 0.171107 * I,
            -0.439107 + 0.439107 * I,
            -0.290677 + 0.484461 * I,
            -0.0680307 + 0.158738 * I,
        ],
        vec![
            0.134 + 0.1876 * I,
            -0.237077 - 0.237077 * I,
            -0.484461 - 0.290677 * I,
            -0.608153 - 0.121631 * I,
            -0.608153 + 0.121631 * I,
            -0.484461 + 0.290677 * I,
            -0.237077 + 0.237077 * I,
            0.134 - 0.1876 * I,
        ],
        vec![
            FRAC_1_SQRT_2 + FRAC_1_SQRT_2 * I,
            0.1876 + 0.134 * I,
            -0.158738 - 0.0680307 * I,
            -0.331907 - 0.0474153 * I,
            -0.331907 + 0.0474153 * I,
            -0.158738 + 0.0680307 * I,
            0.1876 - 0.134 * I,
            FRAC_1_SQRT_2 - FRAC_1_SQRT_2 * I,
        ],
    ]
}

pub fn matrix_zernike_3_3() -> Vec<Vec<Complex<f64>>> {
    vec![
        vec![
            FRAC_1_SQRT_2 + FRAC_1_SQRT_2 * I,
            0.1876 + 0.628768 * I,
            -0.158738 + 0.426738 * I,
            -0.331907 + 0.150492 * I,
            -0.331907 - 0.150492 * I,
            -0.158738 - 0.426738 * I,
            0.1876 - 0.628768 * I,
            FRAC_1_SQRT_2 - FRAC_1_SQRT_2 * I,
        ],
        vec![
            0.628768 + 0.1876 * I,
            0.257692 + 0.257692 * I,
            0.0103077 + 0.204092 * I,
            -0.113384 + 0.0762768 * I,
            -0.113384 - 0.0762768 * I,
            0.0103077 - 0.204092 * I,
            0.257692 - 0.257692 * I,
            0.628768 - 0.1876 * I,
        ],
        vec![
            0.426738 - 0.158738 * I,
            0.204092 + 0.0103077 * I,
            0.0556615 + 0.0556615 * I,
            -0.0185538 + 0.0268 * I,
            -0.0185538 - 0.0268 * I,
            0.0556615 - 0.0556615 * I,
            0.204092 - 0.0103077 * I,
            0.426738 + 0.158738 * I,
        ],
        vec![
            0.150492 - 0.331907 * I,
            0.0762768 - 0.113384 * I,
            0.0268 - 0.0185538 * I,
            0.00206154 + 0.00206154 * I,
            0.00206154 - 0.00206154 * I,
            0.0268 + 0.0185538 * I,
            0.0762768 + 0.113384 * I,
            0.150492 + 0.331907 * I,
        ],
        vec![
            -0.150492 - 0.331907 * I,
            -0.0762768 - 0.113384 * I,
            -0.0268 - 0.0185538 * I,
            -0.00206154 + 0.00206154 * I,
            -0.00206154 - 0.00206154 * I,
            -0.0268 + 0.0185538 * I,
            -0.0762768 + 0.113384 * I,
            -0.150492 + 0.331907 * I,
        ],
        vec![
            -0.426738 - 0.158738 * I,
            -0.204092 + 0.0103077 * I,
            -0.0556615 + 0.0556615 * I,
            0.0185538 + 0.0268 * I,
            0.0185538 - 0.0268 * I,
            -0.0556615 - 0.0556615 * I,
            -0.204092 - 0.0103077 * I,
            -0.426738 + 0.158738 * I,
        ],
        vec![
            -0.628768 + 0.1876 * I,
            -0.257692 + 0.257692 * I,
            -0.0103077 + 0.204092 * I,
            0.113384 + 0.0762768 * I,
            0.113384 - 0.0762768 * I,
            -0.0103077 - 0.204092 * I,
            -0.257692 - 0.257692 * I,
            -0.628768 - 0.1876 * I,
        ],
        vec![
            -FRAC_1_SQRT_2 + FRAC_1_SQRT_2 * I,
            -0.1876 + 0.628768 * I,
            0.158738 + 0.426738 * I,
            0.331907 + 0.150492 * I,
            0.331907 - 0.150492 * I,
            0.158738 - 0.426738 * I,
            -0.1876 - 0.628768 * I,
            -FRAC_1_SQRT_2 - FRAC_1_SQRT_2 * I,
        ],
    ]
}
