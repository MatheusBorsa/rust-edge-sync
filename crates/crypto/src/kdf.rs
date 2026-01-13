use hkdf::Hkdf;
use sha2::Sha256;

pub struct SessionKeys {
    pub send_key: [u8; 32],
    pub recv_key: [u8; 32],
}

//avoids key reuse in both directions
pub fn derive_session_keys(shared_secret: &[u8; 32], is_client: bool) -> SessionKeys {
    let hk = Hkdf::<Sha256>::new(None, shared_secret);

    let mut key_material = [0u8; 64];
    hk.expand(b"edge-sync-session-keys", &mut key_material)
        .expect("HKDF expand failed");

    let (k1, k2) = key_material.split_at(32);

    if is_client {
        SessionKeys {
            send_key: k1.try_into().unwrap(),
            recv_key: k2.try_into().unwrap(),
        }
    } else {
        SessionKeys {
            send_key: k2.try_into().unwrap(),
            recv_key: k1.try_into().unwrap(),
        }
    }
}
