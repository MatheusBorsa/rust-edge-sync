use std::io;
use std::net::TcpStream;

use protocol::{Frame, codec, framer};

use crate::handshake::perform_handshake;

pub fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let mut crypto = perform_handshake(&mut stream)?;

    loop {
        let encrypted = match framer::read_frame(&mut stream) {
            Ok(data) => data,
            Err(ref e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                println!("Client disconnected cleanly");
                break;
            }
            Err(e) => {
                eprintln!("Frame read error: {}", e);
                break;
            }
        };

        let plaintext = crypto
            .decrypt(&encrypted)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "decrypt failed"))?;

        let frame = codec::decode(&plaintext);

        let (id, version) = match frame {
            Frame::Request { id, version, .. } => (id, version),
            _ => continue,
        };

        let response = Frame::Response {
            id,
            version,
            response: protocol::Response::Pong,
        };

        let payload = codec::encode(&response);
        let encrypted = crypto
            .encrypt(&payload)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "encryption failed"))?;

        framer::write_frame(&mut stream, &encrypted)?;
    }

    Ok(())
}
