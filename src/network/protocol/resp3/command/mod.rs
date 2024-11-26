use std::fmt::Debug;
use crate::network::manager::ConnectionManager;

pub mod echo;
pub mod hey;

pub trait Command: Debug {
    fn some_fn(&self, connection_manager: &ConnectionManager);
}