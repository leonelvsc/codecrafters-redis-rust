use std::collections::HashMap;
use std::ptr::null;
use std::rc::Rc;
use crate::network::manager::ConnectionManager;
use crate::network::protocol::resp3::command::Command;
use crate::network::protocol::resp3::command::echo::EchoRequest;
use crate::network::protocol::resp3::command::hey::HeyRequest;

pub mod command;

pub struct RSP3 {
    commands: HashMap<char, Box<dyn Command>>
}

impl RSP3 {
    pub fn new() -> RSP3 {
        let mut hash_map: HashMap<char, Box<dyn Command>> = HashMap::new();
        hash_map.insert('*', Box::new(EchoRequest));
        hash_map.insert('$', Box::new(HeyRequest));
        RSP3 {
            commands: hash_map
        }
    }

    pub fn proccess_line(&self, line: &String, connection_manager: &ConnectionManager) {

        println!("Request: {line:#?}");

        //TODO refactor para instanciar comandos y el protocolo
        if line == "ECHO" {
            let value = self.commands.get(&'*').unwrap();
            println!("HashMapValue: {value:#?}");
            value.some_fn(connection_manager);
        }
    }
}