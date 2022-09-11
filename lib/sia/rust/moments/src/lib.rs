use feature::FeaturesCalculator;
use ndarray::Array2;
use num::Complex;

pub trait MomentFeaturesCalculator: FeaturesCalculator {
    fn get_order_by_features_amount(&self, features_amount: usize) -> usize;
    fn calculate_moment(&self, image: &Array2<u8>, index: usize, order: usize) -> Complex<f64>;
}
