use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::ptr::null;
use std::rc::Rc;
use crate::network::manager::ConnectionManager;
use crate::network::protocol::Protocol;
use crate::network::protocol::resp3::command::Command;
use crate::network::protocol::resp3::command::echo::EchoRequest;
use crate::network::protocol::resp3::command::nil::NilRequest;
use crate::network::protocol::resp3::command::ping::PingRequest;
use crate::network::protocol::resp3::data_types::array::Array;
use crate::network::protocol::resp3::data_types::bulk_string::BulkString;
use crate::network::protocol::resp3::data_types::DataType;
use crate::network::protocol::resp3::data_types::nil::Nil;

pub mod command;
pub mod data_types;

pub struct RSP3<W: Write> {
    data_types: HashMap<char, Rc<RefCell<Box<dyn DataType>>>>,
    commands: HashMap<String, Rc<RefCell<Box<dyn Command<W>>>>>,
    current_command: Rc<RefCell<Box<dyn Command<W>>>>,
    current_data_type: Rc<RefCell<Box<dyn DataType>>>,
}

impl<W: Write + 'static> Protocol<W> for RSP3<W> {
    fn proccess_line(&mut self, line: &String, writer: &mut W) {

        // if TypeId::of::<Nil>() == self.current_data_type.borrow().type_id() {
        //     let data_type = self.data_types.get(&line[0..1].chars().nth(0).unwrap());
        //
        //     self.current_data_type = Rc::clone(&data_type.unwrap());
        //     self.current_data_type.borrow_mut().process_line(&line[1..]);
        // } else {
        //     self.current_data_type.borrow_mut().process_line(line);
        // }

        if !self.current_command.borrow().has_pending_read() {
            self.current_command = Rc::clone(self.commands.get(line).unwrap_or(&self.current_command));
        } else {
            self.current_command.borrow_mut().process_line(line);
        }
        
        if !self.current_data_type.borrow().has_pending_read() {
            self.current_command.borrow().process(writer);
        }

        println!("Request: {line:#?}");
    }
}

impl<W: Write> RSP3<W> {
    pub fn new() -> RSP3<W> {
        let mut data_types: HashMap<char, Rc<RefCell<Box<dyn DataType>>>> = HashMap::new();
        data_types.insert('*', Rc::new(RefCell::new(Box::new(Array::new()))));
        data_types.insert('$', Rc::new(RefCell::new(Box::new(BulkString::new()))));

        let mut commands: HashMap<String, Rc<RefCell<Box<dyn Command<W>>>>> = HashMap::new();
        commands.insert("ECHO".to_string(), Rc::new(RefCell::new(Box::new(EchoRequest::new()))));
        commands.insert("PING".to_string(), Rc::new(RefCell::new(Box::new(PingRequest::new()))));


        RSP3 {
            data_types,
            commands,
            current_command: Rc::new(RefCell::new(Box::new(NilRequest::new()))),
            current_data_type: Rc::new(RefCell::new(Box::new(Nil::new()))),
        }
    }
}