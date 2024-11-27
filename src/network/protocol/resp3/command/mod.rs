use std::any::Any;
use std::fmt::Debug;
use std::io::Write;
use crate::network::protocol::HasPendingRead;

pub mod echo;
pub mod ping;
pub mod nil;

pub trait Command<W: Write>: Debug + Any + HasPendingRead {
    fn process_line(&mut self, line: &String);
    fn process(&self, writer: &mut W);
}