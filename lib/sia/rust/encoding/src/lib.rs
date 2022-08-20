extern crate bitvec;
extern crate labrador_ldpc;

use bitvec::prelude::BitVec;
use bitvec::prelude::*;
use labrador_ldpc::LDPCCode;

fn bytes_to_u16(byte1: u8, byte2: u8) -> u16 {
    ((byte1 as u16) << 8) | byte2 as u16
}

fn u16_to_bytes(number: u16) -> (u8, u8) {
    let second: u8 = ((number >> 8) & 0xff) as u8;
    let first: u8 = (number & 0xff) as u8;
    return (second, first);
}

pub fn get_code_by_codeword_length(codeword_length: usize) -> Option<LDPCCode> {
    match codeword_length * 8 {
        128 => Option::Some(LDPCCode::TC128),
        256 => Option::Some(LDPCCode::TC256),
        512 | 1024 => Option::Some(LDPCCode::TC512),
        2048 | 4096 => Option::Some(LDPCCode::TM2048),
        8192..=usize::MAX => Option::Some(LDPCCode::TM8192),
        _ => Option::None,
    }
}

pub fn get_code_for_encode(length: usize) -> Option<LDPCCode> {
    match length {
        9..=64 => Option::Some(LDPCCode::TC128),
        65..=128 => Option::Some(LDPCCode::TC256),
        129..=256 => Option::Some(LDPCCode::TC512),
        257..=1024 => Option::Some(LDPCCode::TM2048),
        1025..=usize::MAX => Option::Some(LDPCCode::TM8192),
        _ => Option::None,
    }
}

fn get_code_for_decode(length: usize) -> Option<LDPCCode> {
    match length {
        128..=255 => Option::Some(LDPCCode::TC128),
        256..=511 => Option::Some(LDPCCode::TC256),
        512..=2047 => Option::Some(LDPCCode::TC512),
        2048..=8191 => Option::Some(LDPCCode::TM2048),
        8192..=usize::MAX => Option::Some(LDPCCode::TM8192),
        _ => Option::None,
    }
}

pub fn encode(input: Vec<u8>, codeword_length: usize) -> Vec<u8> {
    // Prepend input data with amount of bytes
    let bytes_length = input.len() as u16;
    let (length_byte1, length_byte2) = u16_to_bytes(bytes_length);
    let mut bytes = vec![length_byte1, length_byte2];
    bytes.extend_from_slice(input.as_slice());

    println!("bytes to be encoded with length header: {:?}", bytes);

    // Encode bytes
    let bits: BitVec<u8, Lsb0> = BitVec::from_vec(bytes.clone());
    let bits_length = bits.len();
    println!("binary length of the data to be encoded: {:?}", bits_length);

    //let code_option = get_code_for_encode(bits_length);
    let code_option = get_code_by_codeword_length(codeword_length);

    match code_option {
        None => bytes.clone(),
        Some(code) => {
            let mut encoded_bytes = vec![];
            let words = bytes.chunks(code.k() / 8);
            println!("use LDPC code: ({:?}, {:?})", code.k(), code.n());
            println!("words to be encoded: {:?}", words.len());

            for word in words {
                let mut word_bits: BitVec<u8, Lsb0> = BitVec::from_slice(word);
                word_bits = pad_bits(&word_bits, code.k());
                let padded_word_bytes = word_bits.into_vec();

                //println!("word: {:?}", padded_word_bytes);

                let mut encoded_word_bytes = vec![0u8; code.n() / 8];
                code.copy_encode(
                    padded_word_bytes.clone().as_slice(),
                    &mut encoded_word_bytes,
                );
                //println!("encoded word: {:?}", encoded_word_bytes);
                encoded_bytes.append(&mut encoded_word_bytes);
            }

            //println!("encoded bytes: {:?}", encoded_bytes);
            encoded_bytes
        }
    }
}

fn pad_bits(input: &BitVec<u8>, length: usize) -> BitVec<u8> {
    let mut output = input.clone();
    output.resize(length, false);
    output
}

pub fn decode(bytes: Vec<u8>, codeword_length: usize) -> Vec<u8> {
    let bits: BitVec<u8, Lsb0> = BitVec::from_vec(bytes.clone());

    let bits_length = bits.len();
    println!("bits length: {:?}", bits_length);

    // let code_option = get_code_for_decode(bits_length);
    let code_option = get_code_by_codeword_length(codeword_length);
    match code_option {
        None => bytes.clone(),
        Some(code) => {
            let mut decoded_bytes = vec![];
            let code_words = bytes.chunks(code.n() / 8);
            println!("use LDPC code: ({:?}, {:?})", code.k(), code.n());
            println!("codewords to be decoded: {:?}", code_words.len());

            for code_word in code_words {
                //println!("code word: {:?}", code_word);

                let mut working = vec![0u8; code.decode_bf_working_len()];
                let mut rx_data = vec![0u8; code.output_len()];
                code.decode_bf(code_word, &mut rx_data, &mut working, 20);
                let mut decoded_word_bytes = Vec::from(&rx_data[0..code.k() / 8]);

                //println!("decoded word: {:?}", decoded_word_bytes);
                decoded_bytes.append(&mut decoded_word_bytes);
            }

            // Get data bytes length
            let bytes_length = bytes_to_u16(decoded_bytes[0], decoded_bytes[1]) as usize;

            println!("expected data bytes length: {:?}", bytes_length);

            // Return decoded data
            let decoded_bytes = Vec::from(&decoded_bytes[2..bytes_length + 2]);

            //println!("decoded bytes: {:?}", decoded_bytes);
            decoded_bytes
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitvec::bitvec;

    #[test]
    fn pad_bits_test() {
        let bit_vec = bitvec![u8, Lsb0; 0,1,1];
        let padded = pad_bits(&bit_vec, 5);
        assert_eq!(bitvec![u8, Lsb0; 0,1,1,0,0], padded);
    }

    #[test]
    fn encode_decode_1_bytes_test() {
        let bytes: Vec<u8> = vec![17];
        let encoded = encode(bytes.clone(), 128);
        let decoded = decode(encoded.clone(), 128);
        assert_eq!(bytes, decoded);
    }

    #[test]
    fn encode_decode_2_bytes_test() {
        let bytes: Vec<u8> = vec![17, 233];
        let encoded = encode(bytes.clone(), 128);
        let decoded = decode(encoded.clone(), 128);
        assert_eq!(bytes, decoded);
    }

    #[test]
    fn encode_decode_8_bytes_test() {
        let bytes: Vec<u8> = (0..8).collect();
        let encoded = encode(bytes.clone(), 128);
        let decoded = decode(encoded.clone(), 128);
        assert_eq!(bytes, decoded);
    }

    #[test]
    fn encode_decode_15_bytes_test() {
        let bytes: Vec<u8> = (0..15).collect();
        let encoded = encode(bytes.clone(), 128);
        let decoded = decode(encoded.clone(), 128);
        assert_eq!(bytes, decoded);
    }

    #[test]
    fn encode_decode_254_bytes_test() {
        let bytes: Vec<u8> = vec![
            33, 64, 12, 166, 19, 47, 250, 1, 140, 113, 227, 127, 13, 239, 134, 37, 188, 193, 198,
            19, 39, 52, 102, 210, 180, 193, 160, 225, 185, 70, 69, 182, 71, 184, 245, 150, 192, 93,
            215, 0, 49, 239, 90, 0, 238, 233, 118, 197, 116, 140, 242, 50, 214, 220, 203, 162, 224,
            5, 18, 134, 102, 95, 230, 141, 18, 133, 37, 176, 120, 93, 13, 1, 72, 101, 108, 108,
            111, 32, 119, 111, 114, 108, 100, 33,
        ];
        let encoded = encode(bytes.clone(), 256);
        let decoded = decode(encoded.clone(), 256);
        assert_eq!(bytes, decoded);
    }

    #[test]
    fn various_length_encode_decode_test() {
        let test_lengths: Vec<u16> = vec![8, 64, 65, 128, 129, 256, 257, 1024, 1025, 4096];
        for test_length in test_lengths {
            let bytes: Vec<u8> = (0..test_length).map(|x| (x % 256) as u8).collect();
            let encoded = encode(bytes.clone(), 4096);
            let decoded = decode(encoded.clone(), 4096);
            assert_eq!(bytes, decoded);
        }
    }

    #[test]
    fn simple_encode_decode_test_2() {
        let code = LDPCCode::TC128;
        let tx_data: Vec<u8> = (0..8).collect();
        let mut tx_code = vec![0u8; code.n() / 8];

        code.copy_encode(&tx_data, &mut tx_code);

        let mut rx_code = tx_code.clone();
        rx_code[0] ^= 0x55;

        let mut working = vec![0u8; code.decode_bf_working_len()];
        let mut rx_data = vec![0u8; code.output_len()];

        code.decode_bf(&rx_code, &mut rx_data, &mut working, 20);
        assert_eq!(&rx_data[..8], &tx_data[..8]);
    }

    #[test]
    fn get_code_test() {
        assert_eq!(Option::None, get_code_for_encode(1));

        assert_eq!(LDPCCode::TC128, get_code_for_encode(9).unwrap());
        assert_eq!(LDPCCode::TC128, get_code_for_encode(64).unwrap());

        assert_eq!(LDPCCode::TC256, get_code_for_encode(65).unwrap());
        assert_eq!(LDPCCode::TC256, get_code_for_encode(128).unwrap());

        assert_eq!(LDPCCode::TC512, get_code_for_encode(129).unwrap());
        assert_eq!(LDPCCode::TC512, get_code_for_encode(256).unwrap());

        assert_eq!(LDPCCode::TM2048, get_code_for_encode(257).unwrap());
        assert_eq!(LDPCCode::TM2048, get_code_for_encode(1024).unwrap());

        assert_eq!(LDPCCode::TM8192, get_code_for_encode(1025).unwrap());
        assert_eq!(LDPCCode::TM8192, get_code_for_encode(4096).unwrap());
    }

    #[test]
    fn bytes_to_u16_conversion_test() {
        let mut test_number = 65535;
        let (byte1, byte2) = u16_to_bytes(test_number);
        assert_eq!(255, byte1);
        assert_eq!(255, byte2);

        let number = bytes_to_u16(byte1, byte2);
        assert_eq!(test_number, number);

        test_number = 31411;
        let (byte1, byte2) = u16_to_bytes(test_number);
        assert_eq!(122, byte1);
        assert_eq!(179, byte2);

        let number = bytes_to_u16(byte1, byte2);
        assert_eq!(test_number, number);
    }
}
