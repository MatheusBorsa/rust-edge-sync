use std::io::{Read, Write};
use std::net::TcpStream;

use crypto::key_exchange::KeyPair;
use crypto::session::SessionCrypto;

pub fn perform_handshake(stream: &mut TcpStream) -> std::io::Result<SessionCrypto> {
    let server_kp = KeyPair::generate();

    let mut client_pub = [0u8; 32];
    stream.read_exact(&mut client_pub)?;

    stream.write_all(server_kp.public.as_bytes())?;

    let shared = server_kp.diffie_hellman(&crypto::PublicKey::from(client_pub));

    Ok(SessionCrypto::new(&shared))
}
