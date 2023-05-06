use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use bytes::Bytes;

pub mod core;
pub mod server;
pub mod client;

pub type Storage = Arc<Mutex<HashMap<String, Bytes>>>;

pub const SOCKET_ADDR: &str = "127.0.0.1:6379";