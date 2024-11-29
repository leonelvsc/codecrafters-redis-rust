use bytes::Bytes;
use std::any::Any;
use std::fmt::Debug;
use std::io::Write;

pub mod echo;
pub mod ping;
pub mod nil;
pub mod data_wrapper;

pub trait Command: Debug + Send + Any {
    fn process(&self) -> String;
    fn needs_more_reading(&self) -> bool;
    fn set_data(&mut self, data: Bytes);
    fn get_data(&self) -> Bytes;
}