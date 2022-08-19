pub fn add(left: usize, right: usize) -> usize {
    1
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;
    use sha3::{Digest, Sha3_224};

    #[test]
    fn it_works() {
        let mut hasher = Sha3_224::new();

        hasher.update(b"abc");
        let result = hasher.finalize();

        assert_eq!(
            result[..],
            hex!("e642824c3f8cf24ad09234ee7d3c766fc9a3a5168d0c94ad73b46fdf")[..]
        );
    }
}
