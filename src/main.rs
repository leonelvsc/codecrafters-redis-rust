#![allow(unused_imports)]

mod network;

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};

use network::manager::{ConnectionManager, protocol::tcp::Protocol};
use network::protocol::recp3::command::echo::{EchoRequest};

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let manager = ConnectionManager::new();
    let echo = EchoRequest::new();
    let tcp_protocol = Protocol::new();
    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                tokio::spawn(async move {
                    handle_new_stream(&_stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_new_stream(mut stream: &TcpStream) {
    let buf_reader = BufReader::new(stream);

    println!("Reading...");

    let all_requests: Vec<_> = buf_reader
        .lines()
        .map(|l| {
            let mut line: String = String::new();

            match l {
                Ok(_l) => line = _l,
                Err(e) => {
                    println!("error: {}", e);
                }
            };


            println!("Request: {line:#?}");
            line
        })
        .filter(|line| line.chars().all(|c| c.is_ascii_alphabetic()))
        .map(|l| {
            println!("Request: {l}");

            stream
                .write_all(b"+PONG\r\n")
                .expect("could not write");

            l
        })
        .take_while(|l| !l.is_empty())
        .collect();

    println!("Request: {all_requests:#?}");
}
