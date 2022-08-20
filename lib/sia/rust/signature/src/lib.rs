use ed25519::Signature;
use ed25519_dalek::{Keypair, PublicKey};

use utils::format::decode_hex;

use crate::ecdsa::{DalekSigner, DalekVerifier};

mod ecdsa;
mod keypair;

pub type DSSigner = DalekSigner<Keypair>;
pub type DSVerifier = DalekVerifier<PublicKey>;

pub fn sign(data: &Vec<u8>, private_key: &str, public_key: &str) -> Vec<u8> {
    let private_key_bytes_vector = decode_hex(private_key).unwrap();
    let public_key_bytes_vector = decode_hex(public_key).unwrap();

    let private_key_bytes = private_key_bytes_vector.as_slice();
    let public_key_bytes = public_key_bytes_vector.as_slice();

    let key_pair_bytes = [private_key_bytes, public_key_bytes].concat();
    let key_pair: Keypair = Keypair::from_bytes(key_pair_bytes.as_slice()).unwrap();

    let signer = DSSigner { key_pair };
    let signature = signer.sign(data.as_slice());
    signature.to_bytes().to_vec()
}

pub fn verify(data: &Vec<u8>, signature: &Vec<u8>, public_key: &str) -> bool {
    let public_key_bytes_vector = decode_hex(public_key).unwrap();
    let public_key_bytes = public_key_bytes_vector.as_slice();

    let verify_key = PublicKey::from_bytes(public_key_bytes).unwrap();

    let verifier = DSVerifier { verify_key };

    let sig = Signature::from_bytes(signature.as_slice()).unwrap();
    verifier.verify(data.as_slice(), &sig).is_ok()
}

#[cfg(test)]
mod tests {
    use ed25519_dalek::{Keypair, PublicKey};

    use crate::ecdsa::{DalekSigner, DalekVerifier};
    use crate::keypair::generate;

    use super::*;

    extern crate ed25519_dalek;

    #[test]
    fn signature_test() {
        let (private_key, public_key) = generate();
        println!("generated keypair ({:?}, {:?})", private_key, public_key);

        let data = vec![12, 34, 56, 78];
        let signature = sign(&data, private_key.as_str(), public_key.as_str());
        println!("signature {:?}", signature);

        let is_authentic = verify(&data, &signature, public_key.as_str());
        assert!(is_authentic);
    }
}
