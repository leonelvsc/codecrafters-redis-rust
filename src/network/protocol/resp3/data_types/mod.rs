use std::any::Any;
use std::fmt::Debug;
use std::io::Write;

pub mod bulk_string;
pub mod array;
pub mod nil;

pub trait DataType: Debug + Any {
    fn process_line(&mut self, line: &str);
}