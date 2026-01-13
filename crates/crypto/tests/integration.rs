use crypto::kdf::derive_session_keys;
use crypto::key_exchange::KeyPair;
use crypto::session::SessionCrypto;

#[test]
fn full_handshake_and_encrypted_exchange() {
    let client_kp = KeyPair::generate();
    let server_kp = KeyPair::generate();

    let client_public = client_kp.public;
    let server_public = server_kp.public;

    let client_shared = client_kp.diffie_hellman(&server_public);
    let server_shared = server_kp.diffie_hellman(&client_public);

    assert_eq!(client_shared, server_shared);

    let client_keys = derive_session_keys(&client_shared, true);
    let server_keys = derive_session_keys(&server_shared, false);

    let mut client_crypto = SessionCrypto::new(&client_keys.send_key);
    let mut server_crypto = SessionCrypto::new(&server_keys.recv_key);

    let plaintext = b"ping";

    let encrypted = client_crypto.encrypt(plaintext).unwrap();
    let decrypted = server_crypto.decrypt(&encrypted).unwrap();

    assert_eq!(plaintext.to_vec(), decrypted);
}
