use crypto::key_exchange::KeyPair;

#[test]
fn diffie_hellman_produces_same_shared_secret() {
    let alice = KeyPair::generate();
    let bob = KeyPair::generate();

    let alice_pub = alice.public;
    let bob_pub = bob.public;

    let alice_shared = alice.diffie_hellman(&bob_pub);
    let bob_shared = bob.diffie_hellman(&alice_pub);

    assert_eq!(alice_shared, bob_shared);
}

#[test]
fn diffie_hellman_produces_different_secrets() {
    let bob = KeyPair::generate();
    let eve = KeyPair::generate();

    let alice_ab = KeyPair::generate();
    let alice_ae = KeyPair::generate();

    let shared_ab = alice_ab.diffie_hellman(&bob.public);
    let shared_ae = alice_ae.diffie_hellman(&eve.public);
    assert_ne!(shared_ab, shared_ae);
}
