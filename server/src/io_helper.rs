use crate::cipher::*;
// use crate::io::Error;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::*;
use std::net::TcpStream;

pub fn handle_request(sock: &mut TcpStream) -> Result<()> {
    let msg_str: String = convert_to_string(read_token(sock));
    let key_str: String = convert_to_string(read_token(sock));

    println!("Message to encrypt: {}", msg_str);
    println!("Key: {}", key_str);

    let msg: &str = &msg_str;
    let key: &str = &key_str;

    // encrypt message
    let encrypted_msg = encrypt(msg, key);
    let encrypted_res: Vec<String> = vec![encrypted_msg, key_str];

    write_response(sock, &encrypted_res)?;
    Ok(())
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

fn convert_to_string(payload: Result<Vec<u8>>) -> String {
    match payload {
        Ok(bytes) => {
            // Convert bytes to String, replacing invalid UTF-8 with �
            String::from_utf8_lossy(&bytes).into_owned()
        }
        Err(e) => {
            eprintln!("Error reading payload: {}", e);
            String::new() // return empty string on error
        }
    }
}

pub fn write_buffer(sock: &mut TcpStream, buffer: &String) -> Result<()> {
    // Writes length of the message as a 4 byte unsigned integer in BE order
    sock.write_u32::<BigEndian>(buffer.len() as u32)?;
    // // Writes buffer to socket
    sock.write_all(buffer.as_bytes())?;
    Ok(())
}

pub fn write_response(sock: &mut TcpStream, args: &[String]) -> Result<()> {
    write_buffer(sock, &args[0])?;
    write_buffer(sock, &args[1])?;

    Ok(())
}
