/// These types don't convey much information to avoid potential side-channel leakage (e.g. padding oracle).
pub enum CipherErrors {
    /// The cipher used has not been implemented yet or the cipher is not supported
    InvalidCipher,
    /// The data could not be encrypted
    EncryptionError,
    /// The data could not be decrypted
    DecryptionError,
}
