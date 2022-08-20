use bitvec::prelude::{BitVec, Lsb0};
use image::EncodableLayout;
use three_bit_quantization::{perturbation_vector_as_bytes, perturbation_vector_from_bytes};

#[derive(Debug, Clone, PartialEq)]
pub struct Watermark {
    pub perturbation_vector: BitVec<u8>,
    pub signature: Vec<u8>,
    pub text: String,
}

impl Watermark {
    pub fn new(p: BitVec<u8>, s: Vec<u8>, t: String) -> Self {
        Self {
            perturbation_vector: p,
            signature: s,
            text: t,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let p_length = self.perturbation_vector.len();
        if p_length > 255 {
            panic!("too long perturbation: {:?}", p_length);
        }

        let s_length = self.signature.len();
        if s_length > 255 {
            panic!("too long signature: {:?}", s_length);
        }

        let t_length = self.text.as_bytes().len();
        if t_length > 255 {
            panic!("too long text: {:?}", t_length);
        }

        let perturbation_vector_bytes = perturbation_vector_as_bytes(&self.perturbation_vector);
        let text_bytes = self.text.as_bytes();

        let data = [
            &[p_length as u8],
            &[s_length as u8],
            &[t_length as u8],
            perturbation_vector_bytes.as_slice(),
            &self.signature.as_slice(),
            &text_bytes,
        ]
        .concat();
        data
    }
}

pub fn deserialize(bytes: &Vec<u8>) -> Watermark {
    let length = bytes.len();
    if length < 3 {
        panic!("too short data");
    }

    let perturbation_vector_length = *bytes.get(0).unwrap() as usize;
    let signature_length = *bytes.get(1).unwrap() as usize;
    let text_length = *bytes.get(2).unwrap() as usize;

    let p_start = 3;
    let p_end = p_start + (perturbation_vector_length as f64 / 8.).ceil() as usize;
    // println!("p: ({:?} - {:?})", p_start, p_end);

    let s_start = p_end;
    let s_end = s_start + signature_length;
    // println!("s: ({:?} - {:?})", s_start, s_end);

    let t_start = s_end;
    let t_end = t_start + text_length;
    // println!("t: ({:?} - {:?})", t_start, t_end);

    let p: BitVec<u8> =
        perturbation_vector_from_bytes(&bytes[p_start..p_end].to_vec(), perturbation_vector_length);
    let s: Vec<u8> = bytes[s_start..s_end].to_vec();

    let t: String = String::from_utf8(bytes[t_start..t_end].to_vec()).unwrap();

    Watermark::new(p, s, t)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitvec::bitvec;

    #[test]
    fn get_watermark_data_test() {
        let perturbation_vector = bitvec![u8, Lsb0; 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0];
        let signature: Vec<u8> = vec![123, 42, 56, 54];
        let text = String::from("Hello World!");

        let watermark = Watermark::new(perturbation_vector, signature, text);
        println!("watermark: {:?}", watermark);

        let watermark_bytes = watermark.serialize();
        println!("serialized watermark: {:?}", watermark_bytes);

        let deserialized_watermark = deserialize(&watermark_bytes);
        println!("deserialized watermark: {:?}", deserialized_watermark);

        assert_eq!(watermark, deserialized_watermark);
    }
}
