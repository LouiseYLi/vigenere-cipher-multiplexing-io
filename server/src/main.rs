mod args_helper;
mod cipher;
mod globals;
mod io_helper;
mod math;
use args_helper::*;
#[allow(unused_imports)]
use cipher::*;
#[allow(unused_imports)]
use globals::*;
use io_helper::*;
use std::env::args;
use std::io;
use std::net::TcpListener;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() -> io::Result<()> {
    let args: Vec<String> = args().collect();
    let terminate = Arc::new(AtomicBool::new(false));
    let t_clone = terminate.clone();

    let formatted_ip_at_port = match validate_args(&args) {
        Ok(formatted_ip_at_port) => formatted_ip_at_port,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    ctrlc::set_handler(move || {
        println!("\nSIGINT received! Closing program...");
        t_clone.store(true, Ordering::SeqCst);
    })
    .expect("Error setting up Ctrl-C handler");

    // if Path::new(socket_path).exists() {
    //     fs::remove_file(socket_path)?;
    // }
    let listener = TcpListener::bind(&formatted_ip_at_port)?;
    println!("\tServer is listening on {}", &formatted_ip_at_port);

    // let listener = UnixListener::bind(socket_path)?;

    // loop until CTRL+C received
    //      will terminate after fulfilling one last client connection
    while !terminate.load(Ordering::SeqCst) {
        let mut sock = match listener.accept() {
            Ok((socket, _addr)) => {
                println!("\tAccepted client connection");
                // println!("socket: {:?}, addr: {:?}", socket, _addr);
                socket
            }
            Err(e) => {
                eprintln!("Error:1 {}", e);
                std::process::exit(1);
            }
        };
        match handle_request(&mut sock) {
            Ok(()) => {}
            Err(e) => {
                eprintln!("Error:2 {}", e);
                std::process::exit(1);
            }
        }
    }

    println!("Server closed.");
    Ok(())
}
