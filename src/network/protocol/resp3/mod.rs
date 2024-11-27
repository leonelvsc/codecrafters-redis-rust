use std::collections::HashMap;
use std::io::{Read, Write};
use std::ptr::null;
use std::rc::Rc;
use crate::network::manager::ConnectionManager;
use crate::network::protocol::Protocol;
use crate::network::protocol::resp3::command::Command;
use crate::network::protocol::resp3::command::echo::EchoRequest;
use crate::network::protocol::resp3::command::hey::HeyRequest;

pub mod command;

pub struct RSP3<W: Write> {
    commands: HashMap<char, Box<dyn Command<W>>>
}

impl<W: Write> Protocol<W> for RSP3<W> {

    fn proccess_line(&self, line: &String, writer: &mut W) {

        // Acá queda desglosar el protocolo que va linea por linea, el 1er caracter nos indica el tipo de request
        // y luego el comando

        println!("Request: {line:#?}");

        //TODO refactor para instanciar comandos y el protocolo
        if line == "ECHO" {
            let value = self.commands.get(&'*').unwrap();
            println!("HashMapValue: {value:#?}");
            value.some_fn(writer);
        }
    }
}

impl<W: Write> RSP3<W> {
    pub fn new() -> RSP3<W> {
        let mut hash_map: HashMap<char, Box<dyn Command<W>>> = HashMap::new();
        hash_map.insert('*', Box::new(EchoRequest));
        hash_map.insert('$', Box::new(HeyRequest));
        RSP3 {
            commands: hash_map
        }
    }
}