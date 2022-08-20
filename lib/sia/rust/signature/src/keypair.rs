extern crate rand;

use ed25519_dalek::Keypair;
use rand_core::OsRng;
use utils::format::encode_hex;

pub fn generate() -> (String, String) {
    let key_pair: Keypair = Keypair::generate(&mut OsRng);
    (
        encode_hex(key_pair.secret.to_bytes().to_vec().as_slice()),
        encode_hex(key_pair.public.to_bytes().to_vec().as_slice()),
    )
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn signature_test() {
        let keypair = generate();
        assert_eq!(64, keypair.0.len());
        assert_eq!(64, keypair.1.len());
    }
}
