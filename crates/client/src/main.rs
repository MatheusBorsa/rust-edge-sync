mod handshake;

use std::io::{self, ErrorKind};
use std::net::TcpStream;

use protocol::{Frame, Request, Version};
use protocol::{codec, framer};

use crate::handshake::client_handshake;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:9000")?;
    println!("connected to server");

    println!("starting handshake");
    let mut crypto = client_handshake(&mut stream)?;
    println!("handshake complete");

    let frame = Frame::Request {
        id: 1,
        version: Version { major: 1, minor: 0 },
        request: Request::Ping,
    };

    let payload = codec::encode(&frame);

    let encrypted = crypto
        .encrypt(&payload)
        .map_err(|_| io::Error::new(ErrorKind::InvalidData, "encrypt failed"))?;

    framer::write_frame(&mut stream, &encrypted)?;
    println!("ping sent");

    let encrypted_response = framer::read_frame(&mut stream)?;
    let plaintext = crypto
        .decrypt(&encrypted_response)
        .map_err(|_| io::Error::new(ErrorKind::InvalidData, "decrypt failed"))?;

    let response = codec::decode(&plaintext);
    println!("received response: {:?}", response);

    Ok(())
}
