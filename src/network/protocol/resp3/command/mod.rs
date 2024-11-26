use std::fmt::Debug;

pub mod echo;
pub mod hey;

pub trait Command: Debug {
    fn some_fn(self: Box<Self>);
}