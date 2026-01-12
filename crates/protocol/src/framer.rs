use std::io::{self, Read, Write};

pub fn read_frame<R: Read>(stream: &mut R) -> io::Result<Vec<u8>> {
    //creates prefix variable with 4 bytes
    let mut len_buf = [0u8; 4];
    //keeps reading bytes from the stream until all 4 bytes are filled or an error occurs
    stream.read_exact(&mut len_buf)?;
    //converts bytes to number so now it known how many bytes to read
    let length = u32::from_be_bytes(len_buf) as usize;

    //allocates a vector of length bytes
    let mut payload = vec![0u8; length];
    //reads exaclty length bytes from TCP stream
    stream.read_exact(&mut payload)?;

    Ok(payload)
}

pub fn write_frame<W: Write>(stream: &mut W, payload: &[u8]) -> io::Result<()> {
    let length = payload.len() as u32;
    let len_bytes = length.to_be_bytes();

    stream.write_all(&len_bytes)?;
    stream.write_all(payload)?;
    Ok(())
}
