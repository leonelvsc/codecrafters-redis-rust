use std::any::Any;
use std::fmt::Debug;
use std::io::Write;

pub mod echo;
pub mod ping;

pub trait Command {
    fn process(&self) -> &[u8];
}