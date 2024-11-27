use crate::network::protocol::resp3::RSP3;
use crate::network::protocol::Protocol;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::ops::Deref;
use std::rc::Rc;

pub struct ConnectionManager<R: Read, W: Write> {
    writer: W,
    reader: R,
    protocol: Box<dyn Protocol<W>>,
}

impl<R: Read, W: Write + 'static> ConnectionManager<R, W> {
    pub fn new(
        reader: R,
        writer: W,
        protocol: Box<dyn Protocol<W>>,
    ) -> ConnectionManager<R, W> {
        ConnectionManager {
            writer,
            reader,
            protocol
        }
    }

    pub fn listen(&mut self) {
        let buf_reader = BufReader::new(self.reader.by_ref());

        println!("Reading...");

        let all_requests: Vec<_> = buf_reader
            .lines()
            .map(|l| {
                let mut line: String = String::new();

                match l {
                    Ok(ref _l) => self.protocol.proccess_line(_l, self.writer.by_ref()),
                    Err(e) => {
                        println!("error: {}", e);
                    }
                };

                line
            })
            .take_while(|l| !l.is_empty())
            .collect();

        println!("Request: {all_requests:#?}");
    }
}
