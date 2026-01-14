use std::io::{Read, Write};
use std::net::TcpStream;

use crypto::key_exchange::KeyPair;
use crypto::{session::SessionCrypto, PublicKey};

pub fn client_handshake(stream: &mut TcpStream) -> std::io::Result<SessionCrypto> {
    let kp = KeyPair::generate();

    stream.write_all(kp.public.as_bytes())?;

    let mut server_pub = [0u8; 32];
    stream.read_exact(&mut server_pub)?;

    let shared = kp.diffie_hellman(&PublicKey::from(server_pub));

    Ok(SessionCrypto::new(&shared))
}
