use clap::Parser;
use log::error;
use std::io;
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
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                error!("Stream error: {}", e);
            }
        }
    }

    drop(listener);
}

fn handle_client(mut stream: TcpStream) {
    let _ = io::copy(&mut stream.try_clone().unwrap(), &mut stream);
    stream
        .shutdown(Shutdown::Both)
        .expect("shutdown call failed");
}
