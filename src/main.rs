#![allow(unused_imports)]

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                handle_new_stream(&_stream);
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
        .map(|l| l.unwrap())
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
