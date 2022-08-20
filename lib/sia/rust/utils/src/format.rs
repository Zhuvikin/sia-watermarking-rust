use std::{fmt::Write, num::ParseIntError};

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_encode_decode_test() {
        let bytes: [u8; 5] = [12, 34, 56, 78, 90];
        let hex = encode_hex(&bytes);
        println!("hex: {:?}", hex);

        let decoded = decode_hex(hex.clone().as_str()).unwrap();
        let result = decoded.as_slice();
        assert_eq!(bytes, result)
    }
}
