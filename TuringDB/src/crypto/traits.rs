use crate::CipherErrors;
use secrecy::Secret;

pub trait CipherOps {
    /// FIXME add support for `Payload` and removing `as_ref()` as a result
    fn encrypt(&self, plaintext: Secret<Vec<u8>>) -> core::result::Result<Vec<u8>, CipherErrors>;

    fn decrypt(&self, ciphertext: &[u8]) -> core::result::Result<Vec<u8>, CipherErrors>;
}
