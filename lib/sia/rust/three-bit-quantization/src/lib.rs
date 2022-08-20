use bitvec::bitvec;
use bitvec::field::BitField;
use bitvec::macros::internal::funty::Fundamental;
use bitvec::prelude::{BitVec, Lsb0};
use bitvec::slice::BitSlice;
use bitvec::view::BitView;
use std::io::Read;

pub fn perturbation_vector_as_bytes(bits: &BitVec<u8>) -> Vec<u8> {
    let vec1 = bits.iter().collect::<BitVec<u8>>();
    let mut bytes: Vec<u8> = vec![0; (bits.len() as f64 / 8.).ceil() as usize];

    bytes.copy_from_slice(vec1.as_raw_slice());
    bytes
}

pub fn perturbation_vector_from_bytes(
    bytes: &Vec<u8>,
    perturbation_vector_bits: usize,
) -> BitVec<u8, Lsb0> {
    let mut result = bitvec![u8, Lsb0;];
    let mut i: usize = 0;
    for byte in bytes {
        let bits = byte.view_bits::<Lsb0>();

        for bit in bits.iter() {
            if i < perturbation_vector_bits {
                let x = !bit.as_bool();
                result.push(x);
            } else {
                break;
            }
            i += 1;
        }
    }
    result
}

fn quantize(features: &Vec<f64>, step: f64) -> Vec<i64> {
    features
        .iter()
        .map(|feature| (feature / step).floor() as i64)
        .collect()
}

pub fn three_bit_quantization(features: &Vec<f64>, step: f64) -> (Vec<i64>, BitVec<u8, Lsb0>) {
    let quantized: Vec<i64> = quantize(&features, step);
    //println!("quantized: {:?}", quantized);

    let reminder_bits: Vec<bool> = quantized
        .iter()
        .enumerate()
        .map(|(i, quantized_feature)| {
            let reminder = mod_four(*quantized_feature - 1 as i64);
            let (bit0, bit1) = two_bits_from_integer(reminder);
            // println!("reminder: {:?}=({:?} {:?})", reminder, bit0, bit1);

            let feature = features.get(i).unwrap();
            let b: f64 = step * ((*quantized_feature as f64) + 0.5);

            // println!("f: {:?}, qf: {:?}, b: {:?}", feature, quantized_feature, b);
            let bit2 = *feature >= b;

            [bit0, bit1, bit2]
        })
        .flatten()
        .collect();

    // println!("reminder_bits: {:?}", reminder_bits);

    let mut result = bitvec![u8, Lsb0;];
    for bit in reminder_bits {
        result.push(bit);
    }

    (quantized, result)
}

fn mod_four(quantized_feature: i64) -> u8 {
    let result = quantized_feature.rem_euclid(4);
    if result < 0 {
        (result + 4) as u8
    } else {
        result as u8
    }
}

fn integer_from_two_bits(bit0: u8, bit1: u8) -> u8 {
    let mut bits = bitvec![];
    vec![bit1, bit0].iter().for_each(|bit| bits.push(*bit != 0));
    bits.load_be::<u8>()
}

fn two_bits_from_integer(number: u8) -> (bool, bool) {
    let bits = number.view_bits::<Lsb0>();
    let bit0 = !bits.get(1).unwrap().as_bool();
    let bit1 = !bits.get(0).unwrap().as_bool();
    (bit0, bit1)
}

pub fn three_bit_dequantization(
    noised_features: &Vec<f64>,
    given_perturbation_vector: &BitVec<u8>,
    step: f64,
) -> Vec<i64> {
    let (noised_quantized_features, calculated_perturbation_vector) =
        three_bit_quantization(noised_features, step);
    let calculated_perturbation_vector_chunks: Vec<&BitSlice<u8>> = calculated_perturbation_vector
        .chunks(3)
        .map(|chunk| chunk)
        .collect();

    let restored_features = given_perturbation_vector
        .chunks(3)
        .enumerate()
        .map(|(i, given_perturbation_bits)| {
            let calculated_perturbation_bits =
                *calculated_perturbation_vector_chunks.get(i).unwrap();

            let noised_feature: f64 = *(*noised_features).get(i).unwrap();
            //let noised_quantized_feature: i64 = *(*noised_quantized_features).get(i).unwrap();

            let given_bit1 = given_perturbation_bits.get(0).unwrap().as_u8();
            let given_bit2 = given_perturbation_bits.get(1).unwrap().as_u8();
            let given_bit3 = given_perturbation_bits.get(2).unwrap().as_u8();
            let given_p1p2dec = integer_from_two_bits(given_bit1, given_bit2) as i8;

            let calculated_bit1 = calculated_perturbation_bits.get(0).unwrap().as_u8();
            let calculated_bit2 = calculated_perturbation_bits.get(1).unwrap().as_u8();
            let calculated_bit3 = calculated_perturbation_bits.get(2).unwrap().as_u8();
            let calculated_p1p2dec = integer_from_two_bits(calculated_bit1, calculated_bit2) as i8;

            /*println!(
                "given: [{:?}, {:?}, {:?}], p1p2dec: {:?}",
                given_bit1, given_bit2, given_bit3, given_p1p2dec
            );*/

            /*println!(
                "calculated [{:?}, {:?}, {:?}], p1p2dec: {:?}",
                calculated_bit1, calculated_bit2, calculated_bit3, calculated_p1p2dec
            );*/

            let a = if calculated_p1p2dec == (given_p1p2dec - 1).rem_euclid(4) as i8 {
                0
            } else if calculated_p1p2dec == (given_p1p2dec + 1).rem_euclid(4) as i8 {
                1
            } else {
                2
            } as u8;

            /*println!(
                "data: {:?}, {:?}, a: {:?}",
                noised_feature, noised_quantized_feature, a
            );*/

            if a == 0 && given_bit3 == 0 {
                noised_feature + step
            } else if a == 0 && given_bit3 == 1 && calculated_bit3 == 1 {
                noised_feature + step
            } else if a == 1 && given_bit3 == 1 {
                noised_feature - step
            } else if a == 1 && given_bit3 == 0 && calculated_bit3 == 0 {
                noised_feature - step
            } else {
                noised_feature
            }
        })
        .collect();

    quantize(&restored_features, step)
}

#[cfg(test)]
mod tests {
    use crate::{
        integer_from_two_bits, mod_four, perturbation_vector_as_bytes,
        perturbation_vector_from_bytes, quantize, three_bit_dequantization, three_bit_quantization,
        two_bits_from_integer,
    };
    use bitvec::bitvec;
    use bitvec::prelude::Lsb0;
    use utils::pseudo_random::{Generate, PseudoRandom};

    #[test]
    fn quantization_short_test() {
        let mut random = PseudoRandom { seed: 0 };

        let step = 1.0;
        let noise_magnitude = 1.0;

        let features = vec![1.12, 3.73, -26.024, -32.51];
        println!("features: {:?}", features);

        let (quantized_features, perturbation_vector) = three_bit_quantization(&features, step);

        assert_eq!(vec![1, 3, -27, -33], quantized_features);
        assert_eq!(
            bitvec![u8, Lsb0; 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0],
            perturbation_vector
        );

        let noised_features: Vec<f64> = features
            .clone()
            .iter()
            .map(|feature| {
                let noise = random.generate(-step * noise_magnitude, step * noise_magnitude);
                feature + noise
            })
            .collect();
        println!("noised features: {:?}", noised_features);

        let restored_quantized_features =
            three_bit_dequantization(&noised_features, &perturbation_vector, step);

        println!(
            "restored quantized features: {:?}",
            restored_quantized_features
        );

        assert_eq!(quantized_features, restored_quantized_features);
    }

    #[test]
    fn quantization_long_test() {
        let mut random = PseudoRandom { seed: 0 };

        for step in [
            0.03,
            0.125,
            0.256,
            0.5,
            1.0,
            1.125,
            1.256,
            1.5,
            2.25,
            std::f64::consts::E,
            std::f64::consts::PI,
            9.9,
            10.0,
            15.0,
        ] {
            let noise_magnitude = 1.0; // should be between 0 and 1

            let features = (1..1024).map(|_i| random.generate(-10.0, 10.0)).collect();

            let (quantized_features, perturbation_vector) = three_bit_quantization(&features, step);

            println!("quantized features: {:?}", quantized_features);

            let noised_features: Vec<f64> = features
                .clone()
                .iter()
                .map(|feature| {
                    feature + random.generate(-step * noise_magnitude, step * noise_magnitude)
                })
                .collect();
            println!(
                "noised quantized features: {:?}",
                quantize(&noised_features, step)
            );

            let restored_quantized_features =
                three_bit_dequantization(&noised_features, &perturbation_vector, step);

            println!(
                "restored quantized features: {:?}\n",
                restored_quantized_features
            );

            assert_eq!(quantized_features, restored_quantized_features);
        }
    }

    #[test]
    fn integer_from_two_bits_test() {
        assert_eq!(0, integer_from_two_bits(0, 0));
        assert_eq!(1, integer_from_two_bits(0, 1));
        assert_eq!(2, integer_from_two_bits(1, 0));
        assert_eq!(3, integer_from_two_bits(1, 1));
    }

    #[test]
    fn two_bits_from_integer_test() {
        assert_eq!((false, false), two_bits_from_integer(0));
        assert_eq!((false, true), two_bits_from_integer(1));
        assert_eq!((true, false), two_bits_from_integer(2));
        assert_eq!((true, true), two_bits_from_integer(3));
    }

    #[test]
    fn mod_four_test() {
        assert_eq!(0, mod_four(-8));
        assert_eq!(1, mod_four(-7));
        assert_eq!(2, mod_four(-6));
        assert_eq!(3, mod_four(-5));

        assert_eq!(0, mod_four(-4));
        assert_eq!(1, mod_four(-3));
        assert_eq!(2, mod_four(-2));
        assert_eq!(3, mod_four(-1));

        assert_eq!(0, mod_four(0));
        assert_eq!(1, mod_four(1));
        assert_eq!(2, mod_four(2));
        assert_eq!(3, mod_four(3));

        assert_eq!(0, mod_four(4));
        assert_eq!(1, mod_four(5));
        assert_eq!(2, mod_four(6));
        assert_eq!(3, mod_four(7));

        assert_eq!(0, mod_four(8));
    }

    #[test]
    fn bytes_format_test_1() {
        let perturbation_vector = bitvec![u8, Lsb0; 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0];
        println!("perturbation vector bits: {:?}", perturbation_vector);

        let bytes = perturbation_vector_as_bytes(&perturbation_vector);
        println!("perturbation vector bytes: {:?}", bytes);

        let restored = perturbation_vector_from_bytes(&bytes, perturbation_vector.len());
        println!("restored perturbation vector bits: {:?}", restored);

        assert_eq!(&perturbation_vector, &restored);
    }

    #[test]
    fn bytes_format_test_2() {
        let perturbation_vector = bitvec![u8, Lsb0; 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1];
        println!("perturbation vector bits: {:?}", perturbation_vector);

        let bytes = perturbation_vector_as_bytes(&perturbation_vector);
        println!("perturbation vector bytes: {:?}", bytes);

        let restored = perturbation_vector_from_bytes(&bytes, perturbation_vector.len());
        println!("restored perturbation vector bits: {:?}", restored);

        assert_eq!(&perturbation_vector, &restored);
    }
}
