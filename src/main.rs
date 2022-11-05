use clap::Parser;
use log::error;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    port: String,
}

fn main() {
    env_logger::init();

    let args = Cli::parse();
    let port = &args.port;

    let address = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(address).expect("Error on bind");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    handle_client(stream)
                });
            }
            Err(e) => {
                error!("Stream error: {}", e);
            }
        }
    }

    drop(listener);
}


fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 1024];
    while match stream.read(&mut data) {
        Ok(size) => {
            stream.write(&data[0..size]).expect("Error on write");
            true
        },
        Err(_) => {
            error!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}