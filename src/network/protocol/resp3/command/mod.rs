use std::any::Any;
use std::fmt::Debug;
use std::io::Write;

pub mod echo;
pub mod ping;
pub mod nil;

pub trait Command<W: Write>: Debug + Any {
    fn some_fn(&self, writer: &mut W);
}