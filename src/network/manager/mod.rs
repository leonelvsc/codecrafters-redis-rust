use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpStream};
use std::ops::Deref;
use std::rc::Rc;
use crate::network::protocol::resp3::RSP3;

pub struct ConnectionManager {
    stream: TcpStream,
    protocol: RSP3
}

impl ConnectionManager {
    pub fn new(stream: TcpStream) -> ConnectionManager {
        ConnectionManager {
            stream,
            protocol: RSP3::new(),
        }
    }

    pub fn listen(&mut self) {
        let buf_reader = BufReader::new(self.stream.try_clone().unwrap());

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
                
                self.protocol.proccess_line(&line, self);
                
                line
            })
            .take_while(|l| !l.is_empty())
            .collect();

        println!("Request: {all_requests:#?}");    }

    pub fn write_to_stream(&self, string: &str) {
        println!("Response: {string:#?}");
        (&self.stream)
            .write_all(string.as_bytes())
            .expect("Can't write response");
    }
}
