use ed25519::signature::{Signer, Verifier};

pub struct DalekSigner<S>
where
    S: Signer<ed25519::Signature>,
{
    pub key_pair: S,
}

impl<S> DalekSigner<S>
where
    S: Signer<ed25519::Signature>,
{
    pub fn sign(&self, data: &[u8]) -> ed25519::Signature {
        self.key_pair.sign(data)
    }
}

pub struct DalekVerifier<V> {
    pub verify_key: V,
}

impl<V> DalekVerifier<V>
where
    V: Verifier<ed25519::Signature>,
{
    pub fn verify(
        &self,
        data: &[u8],
        signature: &ed25519::Signature,
    ) -> Result<(), ed25519::Error> {
        self.verify_key.verify(data, signature)
    }
}
