use crate::error::CryptoError;
use chacha20poly1305::{
    ChaCha20Poly1305, Key, Nonce,
    aead::{Aead, KeyInit},
};

pub struct SessionCrypto {
    cipher: ChaCha20Poly1305,
    send_nonce: u64,
    recv_nonce: u64,
}

impl SessionCrypto {
    pub fn new(key_bytes: &[u8; 32]) -> Self {
        let key = Key::from_slice(key_bytes);
        let cipher = ChaCha20Poly1305::new(key);

        Self {
            cipher,
            send_nonce: 0,
            recv_nonce: 0,
        }
    }

    fn make_nonce(counter: u64) -> Nonce {
        let mut nonce = [0u8; 12];
        nonce[4..].copy_from_slice(&counter.to_be_bytes());
        Nonce::from_slice(&nonce).clone()
    }

    pub fn encrypt(&mut self, plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let nonce = Self::make_nonce(self.send_nonce);
        self.send_nonce += 1;

        self.cipher
            .encrypt(&nonce, plaintext)
            .map_err(|_| CryptoError::EncryptionFailed)
    }

    pub fn decrypt(&mut self, ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let nonce = Self::make_nonce(self.recv_nonce);
        self.recv_nonce += 1;

        self.cipher
            .decrypt(&nonce, ciphertext)
            .map_err(|_| CryptoError::DecryptionFailed)
    }
}
