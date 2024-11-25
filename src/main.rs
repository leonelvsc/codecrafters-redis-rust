#![allow(unused_imports)]

mod network;

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};

use network::manager::{ConnectionManager};

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                tokio::spawn(async move {
                    ConnectionManager::new(_stream).listen().await;
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
