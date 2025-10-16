use crate::cipher::*;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::*;
// use std::io::prelude::*;
use std::net::TcpStream;

pub fn write_buffer(sock: &mut TcpStream, buffer: &String) -> Result<()> {
    // Writes length of the message as a 4 byte unsigned integer in BE order
    sock.write_u32::<BigEndian>(buffer.len() as u32)?;
    // // Writes buffer to socket
    sock.write_all(buffer.as_bytes())?;

    Ok(())
}

pub fn write_request(sock: &mut TcpStream, args: &[String]) -> Result<()> {
    write_buffer(sock, &args[1])?;
    write_buffer(sock, &args[2])?;

    Ok(())
}

pub fn handle_encryption(sock: &mut TcpStream, key_str: &str) -> Result<()> {
    let msg_str: String = convert_to_string(read_token(sock))?;

    println!("Encrypted message: {}", msg_str);
    println!("Received key: {}", key_str);

    println!("Decrypted message: {}", decrypt(msg_str, key_str));

    Ok(())
}

fn convert_to_string(
    payload: std::result::Result<Vec<u8>, Error>,
) -> std::result::Result<String, std::io::Error> {
    let bytes = payload?;
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

fn read_token(sock: &mut TcpStream) -> Result<Vec<u8>> {
    // Retrieve length of payload
    let payload_len = match sock.read_u32::<BigEndian>() {
        Ok(len) => len as usize,
        Err(e) => return Err(e),
    };
    // Allocate memory for byte buffer of len size
    let mut payload = vec![0u8; payload_len];

    sock.read_exact(&mut payload)?;

    Ok(payload)
}
