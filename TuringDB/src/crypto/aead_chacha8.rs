use crate::{CipherErrors, CipherOps};
use chacha20poly1305::{
    aead::{Aead, NewAead},
    Key, XChaCha8Poly1305, XNonce,
};
use secrecy::{ExposeSecret, Secret};

/// FIXME implement `PartialEq, Eq` for `Cipher` using `blake3::Hash` hashing of fields
/// to provide constant-time eq
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum Cipher {
    XChaCha8Poly1305 {
        key: Secret<[u8; 32]>,
        nonce: Secret<[u8; 24]>,
    },
    XChaCha12Poly1305 {
        key: Secret<[u8; 32]>,
        nonce: Secret<[u8; 24]>,
    },
    XChaCha20Poly1305 {
        key: Secret<[u8; 32]>,
        nonce: Secret<[u8; 24]>,
    },
    XChaCha8Blake3SIV {
        key: Secret<[u8; 32]>,
        nonce: Secret<[u8; 24]>,
    },
    AES256_GCM {
        key: Secret<[u8; 32]>,
        nonce: Secret<[u8; 24]>,
    },
    InvalidCipher,
}

impl secrecy::DebugSecret for Cipher {}

impl CipherOps for Cipher {
    /// FIXME add support for `Payload` and removing `as_ref()` as a result
    fn encrypt(&self, plaintext: Secret<Vec<u8>>) -> core::result::Result<Vec<u8>, CipherErrors> {
        match self {
            Self::XChaCha8Poly1305 { key, nonce } => {
                let key = Key::from_slice(key.expose_secret());
                let nonce = XNonce::from_slice(nonce.expose_secret());

                let aead_encrypt = XChaCha8Poly1305::new(key);

                match aead_encrypt.encrypt(nonce, plaintext.expose_secret().as_ref()) {
                    Ok(ciphertext) => Ok(ciphertext),
                    Err(_) => Err(CipherErrors::EncryptionError),
                }
            }
            _ => Err(CipherErrors::InvalidCipher),
        }
    }

    fn decrypt(&self, ciphertext: &[u8]) -> core::result::Result<Vec<u8>, CipherErrors> {
        match self {
            Self::XChaCha8Poly1305 { key, nonce } => {
                let key = Key::from_slice(key.expose_secret());
                let nonce = XNonce::from_slice(nonce.expose_secret());

                let aead_encrypt = XChaCha8Poly1305::new(key);

                match aead_encrypt.decrypt(nonce, ciphertext) {
                    Ok(plaintext) => Ok(plaintext),
                    Err(_) => Err(CipherErrors::DecryptionError),
                }
            }
            _ => Err(CipherErrors::InvalidCipher),
        }
    }
}
