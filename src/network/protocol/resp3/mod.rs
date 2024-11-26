use std::collections::HashMap;
use std::ptr::null;
use std::rc::Rc;
use crate::network::manager::ConnectionManager;
use crate::network::protocol::resp3::command::Command;
use crate::network::protocol::resp3::command::echo::EchoRequest;

pub mod command;

pub struct RSP3 {
    commands: HashMap<char, Box<dyn Command>>
}

impl RSP3 {
    pub fn new() -> RSP3 {
        let mut hash_map: HashMap<char, Box<dyn Command>> = HashMap::new();
        hash_map.insert('*', Box::new(EchoRequest));
        hash_map.insert('$', Box::new(EchoRequest));
        RSP3 {
            commands: hash_map
        }
    }

    pub fn proccess_line(&self, line: &String, connection_manager: &ConnectionManager) {
        let value = self.commands.get(&line.chars().next().unwrap());
        println!("Request: {line:#?}");
        println!("HashMapValue: {value:#?}");

        //TODO refactor para instanciar comandos y el protocolo
        if line == "ECHO" {
            connection_manager.write_to_stream("+PONG\r\n");
        }
    }
}