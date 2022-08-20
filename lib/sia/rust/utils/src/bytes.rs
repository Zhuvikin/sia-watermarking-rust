pub fn vec_u16_to_bytes(integers: &Vec<u16>) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];
    for integer in integers {
        let bytes = integer.to_le_bytes();
        bytes.iter().for_each(|byte| result.push(*byte))
    }

    result
}

pub fn vec_u16_from_bytes(bytes: &Vec<u8>) -> Vec<u16> {
    let mut result: Vec<u16> = vec![];
    for chunk in bytes.chunks(2) {
        let integer = u16::from_le_bytes(chunk.try_into().unwrap());
        result.push(integer);
    }
    result
}

pub fn vec_i64_to_bytes(integers: &Vec<i64>) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];
    for integer in integers {
        let bytes = integer.to_le_bytes();
        bytes.iter().for_each(|byte| result.push(*byte))
    }

    result
}

pub fn vec_i64_from_bytes(bytes: &Vec<u8>) -> Vec<i64> {
    let mut result: Vec<i64> = vec![];
    for chunk in bytes.chunks(8) {
        let integer = i64::from_le_bytes(chunk.try_into().unwrap());
        result.push(integer);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integers_i64_to_bytes_test() {
        let integers: Vec<i64> = vec![-324324, 45436346, -2132145325, 2, 0, -90];
        println!("integers: {:?}", integers);

        let bytes = vec_i64_to_bytes(&integers);
        println!("bytes: {:?}", bytes);

        let actual = vec_i64_from_bytes(&bytes);
        println!("restored: {:?}", actual);

        assert_eq!(integers, actual)
    }

    #[test]
    fn integers_u16_to_bytes_test() {
        let integers: Vec<u16> = vec![65535, 6555, 5535, 0];
        println!("integers: {:?}", integers);

        let bytes = vec_u16_to_bytes(&integers);
        println!("bytes: {:?}", bytes);

        let actual = vec_u16_from_bytes(&bytes);
        println!("restored: {:?}", actual);

        assert_eq!(integers, actual)
    }
}
