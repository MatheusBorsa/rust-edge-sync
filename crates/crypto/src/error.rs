use thiserror::Error;

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("encryption failed")]
    EncryptionFailed,

    #[error("decryption failed")]
    DecryptionFailed,

    #[error("invalid key material")]
    InvalidKey,

    #[error("nonce reuse detected")]
    NonceReuse,
}
