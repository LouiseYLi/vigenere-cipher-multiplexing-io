use crate::cipher::*;
// use crate::io::Error;

use byteorder::BigEndian;
use byteorder::ByteOrder;
use rand::Rng;
#[allow(unused_imports)]
use rand::SeedableRng;
use rand::rngs::StdRng;
use tokio::io::Result;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{Duration, sleep};

pub async fn handle_request(sock: &mut TcpStream, min_sleep: u16, max_sleep: u16) -> Result<()> {
    let mut rng = StdRng::from_entropy();
    let sleep_duration = Duration::from_secs(rng.gen_range(min_sleep..=max_sleep) as u64);
    println!("Sleeping for {} seconds...", sleep_duration.as_secs());
    sleep(sleep_duration).await;
    println!("Woke up after {} seconds!", sleep_duration.as_secs());

    let msg_str = convert_to_string(read_token(sock).await?);
    let key_str = convert_to_string(read_token(sock).await?);
    println!("Message to encrypt: {}", msg_str);
    println!("Key: {}", key_str);

    let encrypted_msg = encrypt(&msg_str, &key_str);
    let encrypted_res = vec![encrypted_msg];

    write_response(sock, &encrypted_res).await?;

    Ok(())
}

async fn read_token(sock: &mut TcpStream) -> Result<Vec<u8>> {
    // Retrieve length of payload
    let mut len_buf = [0u8; 4];
    sock.read_exact(&mut len_buf).await?;

    let payload_len = u32::from_be_bytes(len_buf) as usize;

    // Allocate memory for byte buffer of len size
    let mut payload = vec![0u8; payload_len];

    sock.read_exact(&mut payload).await?;

    Ok(payload)
}

fn convert_to_string(bytes: Vec<u8>) -> String {
    String::from_utf8_lossy(&bytes).into_owned()
}

async fn write_buffer(sock: &mut TcpStream, buffer: &String) -> Result<()> {
    let mut len_buf = [0u8; 4];
    BigEndian::write_u32(&mut len_buf, buffer.len() as u32);
    // Writes buffer to socket
    sock.write_all(&len_buf).await?;
    // Write buffer asynchronously
    sock.write_all(buffer.as_bytes()).await?;
    Ok(())
}

async fn write_response(sock: &mut TcpStream, args: &[String]) -> Result<()> {
    write_buffer(sock, &args[0]).await?;

    Ok(())
}
