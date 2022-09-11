use ndarray::Array2;

pub trait FeaturesCalculator {
    fn calculate_features(&self, image: &Array2<u8>, features_amount: usize) -> Vec<f64>;
}
