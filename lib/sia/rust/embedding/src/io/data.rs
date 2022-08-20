use bitvec::prelude::BitVec;
use utils::bytes::{vec_u16_from_bytes, vec_u16_to_bytes};

#[derive(Debug, Clone, PartialEq)]
pub struct Data {
    pub bytes: Vec<u8>,
}

impl Data {
    pub fn new(data: Vec<u8>) -> Self {
        Self { bytes: data }
    }

    pub fn serialize(&self) -> Vec<u8> {
        println!("data: {:?}", self);
        let data_length = self.bytes.len();
        if data_length > u16::MAX as usize {
            panic!("too long data: {:?}", data_length);
        }

        let data_length_bytes = vec_u16_to_bytes(&vec![data_length as u16]);

        let data = [data_length_bytes.as_slice(), &self.bytes.as_slice()].concat();
        println!("serialized data bytes: {:?}", data);
        data
    }
}

pub fn deserialize(bytes: &Vec<u8>) -> Data {
    println!("serialized data bytes: {:?}", bytes);
    let length = bytes.len();
    if length < 3 {
        panic!("too short data");
    }

    let data_length_bytes = &bytes[0..2];
    let data_length = *vec_u16_from_bytes(&data_length_bytes.to_vec())
        .get(0)
        .unwrap();
    println!("data length: {:?}", data_length);

    let data_start = 2;
    let data_end = data_start + data_length as usize;
    println!("data bytes: ({:?} - {:?})", data_start, data_end);

    let data_bytes: Vec<u8> = bytes[data_start..data_end].to_vec();
    let data = Data::new(data_bytes);
    println!("deserialized data: {:?}", data);
    data
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitvec::bitvec;

    const TEST_STRING: &'static str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

    #[test]
    fn data_serialization_test() {
        println!("test string: {:?}", TEST_STRING);
        let bytes: Vec<u8> = Vec::from(TEST_STRING);

        let data = Data::new(bytes);
        println!("data: {:?}", data);

        let serialized = data.serialize();
        println!("serialized data: {:?}", serialized);

        let deserialized = deserialize(&serialized);
        println!("deserialized data: {:?}", deserialized);

        assert_eq!(data, deserialized);

        let deserialized_string: String = String::from_utf8(deserialized.bytes).unwrap();
        println!("deserialized string: {:?}", deserialized_string);

        assert_eq!(TEST_STRING, deserialized_string);
    }
}
