use crate::feature_types::FeaturesType;
use feature::FeaturesCalculator;
use ndarray::Array2;
use num::complex::ComplexFloat;
use zernike::ZernikeFeaturesCalculator;

pub mod feature_types;

pub fn calculate_features(
    image: &Array2<u8>,
    features_type: FeaturesType,
    features_amount: usize,
) -> Vec<f64> {
    match features_type {
        FeaturesType::MomentsZernike => {
            let zernike_features = ZernikeFeaturesCalculator {};
            let zernike_moments = zernike_features.calculate_features(image, features_amount);
            println!("Zernike moments: {:?}", zernike_moments);
            zernike_moments
        }
        FeaturesType::MomentsPseudoZernike => {
            vec![]
        }
        FeaturesType::CentralFiniteDifferences => {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::assert_approximately_equals_1d;

    #[test]
    fn calculate_features_test() {
        assert_approximately_equals_1d(
            &vec![0., 0., 0.],
            &calculate_features(
                &utils::vector_2d_as_nd_array(vec![
                    vec![1, 1, 1, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 1, 1, 1, 1, 1],
                ]),
                FeaturesType::MomentsZernike,
                3,
            ),
        );

        assert_approximately_equals_1d(
            &vec![5.58333, 5.10827, 6.80105],
            &calculate_features(
                &utils::vector_2d_as_nd_array(vec![
                    vec![160, 26, 188, 252, 3, 49, 212, 206],
                    vec![240, 126, 183, 47, 40, 49, 111, 168],
                    vec![196, 140, 141, 191, 94, 174, 15, 149],
                    vec![72, 155, 171, 156, 204, 71, 166, 2],
                    vec![244, 164, 164, 65, 51, 137, 185, 123],
                    vec![252, 42, 104, 254, 61, 241, 84, 133],
                    vec![146, 174, 29, 145, 85, 56, 80, 130],
                    vec![114, 67, 163, 80, 165, 129, 101, 42],
                ]),
                FeaturesType::MomentsZernike,
                3,
            ),
        );
    }
}
