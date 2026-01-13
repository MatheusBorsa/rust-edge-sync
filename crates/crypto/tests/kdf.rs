use crypto::kdf::derive_session_keys;

#[test]
fn client_and_server_keys_are_complementary() {
    let shared = [42u8; 32];

    let client = derive_session_keys(&shared, true);
    let server = derive_session_keys(&shared, false);

    assert_eq!(client.send_key, server.recv_key);
    assert_eq!(client.recv_key, server.send_key);
}

#[test]
fn kdf_is_deterministic() {
    let shared = [1u8; 32];

    let a = derive_session_keys(&shared, true);
    let b = derive_session_keys(&shared, true);

    assert_eq!(a.send_key, b.send_key);
    assert_eq!(a.recv_key, b.recv_key);
}

#[test]
fn different_shared_secrets_produce_different_keys() {
    let a = derive_session_keys(&[1u8; 32], true);
    let b = derive_session_keys(&[2u8; 32], true);

    assert_ne!(a.send_key, b.send_key);
}
