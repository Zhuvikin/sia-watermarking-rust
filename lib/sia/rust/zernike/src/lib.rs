pub mod calc;
mod constants;

use crate::calc::calculate_zernike_image_moment;
use feature::FeaturesCalculator;
use moments::MomentFeaturesCalculator;
use ndarray::Array2;
use num::complex::ComplexFloat;
use num::Complex;

pub struct ZernikeFeaturesCalculator {}

impl FeaturesCalculator for ZernikeFeaturesCalculator {
    fn calculate_features(&self, image: &Array2<u8>, features_amount: usize) -> Vec<f64> {
        let order = self.get_order_by_features_amount(features_amount);
        let mut start_index: usize = 0;
        if order % 2 == 1 {
            start_index = 1;
        }
        let moments_indices: Vec<usize> = (start_index..=order).step_by(2).collect();
        println!("Zernike moments indices: {:?}", moments_indices);

        moments_indices
            .iter()
            .map(|index| self.calculate_moment(&image, *index, order).abs())
            .collect()
    }
}

impl MomentFeaturesCalculator for ZernikeFeaturesCalculator {
    fn get_order_by_features_amount(&self, features_amount: usize) -> usize {
        1 + 2 * (features_amount - 1)
    }

    fn calculate_moment(&self, image: &Array2<u8>, index: usize, order: usize) -> Complex<f64> {
        calculate_zernike_image_moment(order, index, image)
    }
}
