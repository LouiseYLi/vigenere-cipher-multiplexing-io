mod args;
mod cipher;
mod globals;
mod io_helper;
mod sock;

use args::*;
#[allow(unused_imports)]
use cipher::*;
#[allow(unused_imports)]
use sock::*;
#[allow(unused_imports)]
use globals::*;
use io_helper::*;
use std::env::args;
use std::io::*;
// use std::io::prelude::*;
use std::net::TcpStream;

// use std::io::{Read, Write};
// std::env::args_os use this version of args if arguments include invalid unicode

fn main() -> Result<()> {
    // collect() turns iterator of arguments into a vector of arguments
    //      since collect() creates many different vectors, we ensure to specify the type here
    let args: Vec<String> = args().collect();

    let formatted_ip_at_port = match validate_args(&args) {
        Ok(formatted_ip_at_port) => {formatted_ip_at_port}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    // let ip_at_port = format!("{}:{}", args[3], args[4]);
    println!("{}", formatted_ip_at_port);
    // if !ip_at_port.is_empty() {
    //     std::process::exit(1);
    // }
    println!("{}", u16::MAX);
    let mut sock = match TcpStream::connect(formatted_ip_at_port) {
        Ok(stream) => {
            println!("Connected to server at {}", &args[3]);
            stream
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match write_request(&mut sock, &args) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    match handle_encryption(&mut sock) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
