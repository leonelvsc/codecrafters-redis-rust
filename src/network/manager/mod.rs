use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpStream};

pub struct ConnectionManager {
    stream: TcpStream
}

impl ConnectionManager {
    pub fn new(stream: TcpStream) -> ConnectionManager {
        ConnectionManager {
            stream
        }
    }

    pub async fn listen(&mut self) {
        let buf_reader = BufReader::new(&self.stream);

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

                //TODO refactor para instanciar comandos y el protocolo
                if line == "ECHO" {
                    self.write_to_stream("+PONG\r\n");
                }
                
                line
            })
            .take_while(|l| !l.is_empty())
            .collect();

        println!("Request: {all_requests:#?}");
    }

    pub fn write_to_stream(&self, string: &str) {
        println!("Response: {string:#?}");
        (&self.stream)
            .write_all(string.as_bytes())
            .expect("Can't write response");
    }
}
