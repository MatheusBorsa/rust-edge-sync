use crypto::error::CryptoError;
use crypto::session::SessionCrypto;

#[test]
fn encrypt_decrypt_roundtrip() {
    let key = [7u8; 32];
    let mut sender = SessionCrypto::new(&key);
    let mut receiver = SessionCrypto::new(&key);

    let plaintext = b"helloworld";

    let ciphertext = sender.encrypt(plaintext).unwrap();
    let decrypted = receiver.decrypt(&ciphertext).unwrap();

    assert_eq!(plaintext.to_vec(), decrypted);
}

#[test]
fn decrypt_with_wrong_key_fails() {
    let mut sender = SessionCrypto::new(&[1u8; 32]);
    let mut receiver = SessionCrypto::new(&[2u8; 32]);

    let ciphertext = sender.encrypt(b"secret").unwrap();
    let result = receiver.decrypt(&ciphertext);

    assert!(matches!(result, Err(CryptoError::DecryptionFailed)));
}

#[test]
fn tampered_ciphertext_fails() {
    let key = [9u8; 32];
    let mut sender = SessionCrypto::new(&key);
    let mut receiver = SessionCrypto::new(&key);

    let mut ciphertext = sender.encrypt(b"attack at dawn").unwrap();
    ciphertext[0] ^= 0xff;

    let result = receiver.decrypt(&ciphertext);
    assert!(result.is_err());
}

#[test]
fn nonce_order_is_enforced() {
    let key = [5u8; 32];
    let mut sender = SessionCrypto::new(&key);
    let mut receiver = SessionCrypto::new(&key);

    let _c1 = sender.encrypt(b"first").unwrap();
    let c2 = sender.encrypt(b"second").unwrap();

    assert!(receiver.decrypt(&c2).is_err());
}

#[test]
fn multiple_messages_roundtrip() {
    let key = [1u8; 32];

    let mut sender = SessionCrypto::new(&key);
    let mut receiver = SessionCrypto::new(&key);

    let messages = [b"message a", b"message b", b"message c"];

    for msg in messages {
        let ct = sender.encrypt(msg).unwrap();
        let pt = receiver.decrypt(&ct).unwrap();
        assert_eq!(msg.to_vec(), pt);
    }
}
