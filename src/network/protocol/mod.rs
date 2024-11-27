use std::io::Write;
use crate::network::manager::ConnectionManager;

pub mod resp3;


pub trait Protocol<W: Write> {
    fn proccess_line(&self, line: &String, writer: &mut W);
}