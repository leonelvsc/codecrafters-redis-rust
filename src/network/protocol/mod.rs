use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Write;
use std::rc::Rc;
use crate::network::manager::ConnectionManager;
use crate::network::protocol::resp3::data_types::DataType;

pub mod resp3;

pub trait HasPendingRead {
    fn has_pending_read(&self) -> bool;
}

pub trait Protocol<W: Write> {
    fn proccess_line(&mut self, line: &String, writer: &mut W);
}