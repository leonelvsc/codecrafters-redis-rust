#![allow(unused_imports)]
extern crate core;

mod network;

use network::manager::ConnectionManager;
use std::io::{BufRead, Read, Write};
use tokio::net::TcpListener;
use std::ops::Deref;

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    
    let listener = TcpListener::bind("127.0.0.1:6379").await.expect("Failed to bind port 6379");

    loop {
        match listener.accept().await {
            Ok((stream, _addr)) => {
                // Spawn a new task to handle the connection
                tokio::spawn(async move {
                    ConnectionManager::new(stream)
                        .listen()
                        .await;
                });
            }
            Err(e) => {
                println!("Error accepting connection: {}", e);
            }
        }
    }
}
