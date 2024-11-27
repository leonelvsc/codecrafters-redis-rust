use std::fmt::Debug;
use std::io::Write;
use crate::network::manager::ConnectionManager;

pub mod echo;
pub mod hey;

pub trait Command<W: Write>: Debug {
    fn some_fn(&self, writer: &mut W);
}