use std::any::Any;
use std::fmt::Debug;
use std::io::Write;
use crate::network::protocol::HasPendingRead;

pub mod simple_string;
pub mod bulk_string;
pub mod array;
pub mod nil;

pub trait DataType: Debug + Any + HasPendingRead {
    fn process_line(&mut self, line: &str);
    fn serialize(&self) -> String;
}