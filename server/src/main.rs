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
use tokio::net::TcpListener;
// use std::net::TcpListener;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[tokio::main]
async fn main() -> io::Result<()> {
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

    let min_delay = get_delay(&args, 3);
    let max_delay = get_delay(&args, 4);

    ctrlc::set_handler(move || {
        println!("\nSIGINT received! Closing program...");
        t_clone.store(true, Ordering::SeqCst);
    })
    .expect("Error setting up Ctrl-C handler");

    let listener = TcpListener::bind(&formatted_ip_at_port).await?;
    println!("\tServer is listening on {}", &formatted_ip_at_port);

    // loop until CTRL+C received
    //      will terminate after fulfilling one last client connection
    while !terminate.load(Ordering::SeqCst) {
        let mut sock = match listener.accept().await {
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
        tokio::spawn(async move {
            match handle_request(&mut sock, min_delay, max_delay).await {
                Ok(()) => {}
                Err(e) => {
                    eprintln!("Error:2 {}", e);
                }
            }
        });
    }

    println!("Server closed.");
    Ok(())
}
