use crate::feature_types::FeaturesType;
use ndarray::Array2;

mod feature_types;
mod moments;

pub fn calculate_features(image: Array2<u8>, features_type: FeaturesType) -> Vec<f64> {
    let mut result: Vec<f64> = vec![];
    match features_type {
        FeaturesType::MomentsZernike => {
            let moment = image.fold(0.0, |accumulator, next| {
                let next_pixel = (*next) as f64;
                println!("a: {:?}, next: {:?}", accumulator, next);
                return accumulator + next_pixel;
            });
            result.push(moment);
        }
        FeaturesType::MomentsPseudoZernike => {}
        FeaturesType::CentralFiniteDifferences => {}
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_features_test() {
        //let image = utils::vector_2d_as_nd_array(vec![
        //    vec![1, 1, 1, 1, 1, 1, 1, 1],
        //    vec![1, 1, 1, 1, 1, 1, 1, 1],
        //    vec![1, 1, 1, 1, 1, 1, 1, 1],
        //    vec![1, 1, 1, 1, 1, 1, 1, 1],
        //    vec![1, 1, 1, 1, 1, 1, 1, 1],
        //    vec![1, 1, 1, 1, 1, 1, 1, 1],
        //    vec![1, 1, 1, 1, 1, 1, 1, 1],
        //    vec![1, 1, 1, 1, 1, 1, 1, 1],
        //]);
        //let features = calculate_features(image, FeaturesType::MomentsZernike);
        //
        //let expected_features = vec![0., 1., 2., 3., 4., 5., 6.];
        //assert_eq!(expected_features, features);
    }
}
