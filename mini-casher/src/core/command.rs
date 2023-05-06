use bytes::Bytes;
use crate::core::error::{CashError, Error};
use crate::core::frames::Frame;

#[derive(Debug)]
pub enum Command {
    Get(Get),
    Set(Set),
    Delete(Get),
    Len,
    All,
    Ping,
}

#[derive(Debug)]
pub struct Get {
    key: String,
}

impl Get {
    pub fn new(key: String) -> Self {
        Self { key }
    }

    pub fn key(&self) -> &String {
        &self.key
    }
}

#[derive(Debug)]
pub struct Set {
    key: String,
    value: Bytes,
}

impl Set {
    pub fn new(key: String, value: Bytes) -> Self {
        Self { key, value }
    }

    pub fn key(&self) -> &String {
        &self.key
    }

    pub fn value(&self) -> &Bytes {
        &self.value
    }
}


impl Command {
    pub fn get_frame(key: &str) -> Frame {
        Frame::Array(vec![
            Frame::BulkString(Bytes::from("get")),
            Frame::BulkString(Bytes::from(key.to_string())),
        ])
    }

    pub fn set_frame(key: &str, value: Bytes) -> Frame {
        Frame::Array(vec![
            Frame::BulkString(Bytes::from("set")),
            Frame::BulkString(Bytes::from(key.to_string())),
            Frame::BulkString(Bytes::from(value.clone())),
        ])
    }

    pub fn delete_frame(key: &str) -> Frame {
        Frame::Array(vec![
            Frame::BulkString(Bytes::from("delete")),
            Frame::BulkString(Bytes::from(key.to_string())),
        ])
    }

    pub fn ping_frame() -> Frame {
        Frame::Array(vec![Frame::BulkString(Bytes::from("ping"))])
    }

    pub fn len_frame() -> Frame {
        Frame::Array(vec![Frame::BulkString(Bytes::from("len"))])
    }

    pub fn all_frame() -> Frame {
        Frame::Array(vec![Frame::BulkString(Bytes::from("all"))])
    }


    pub fn from_cmd(input: String) -> Result<Command, CashError> {
        let input = input.replace("\r\n", "");
        let mut args = input.split(" ");
        let command = args.next().unwrap_or("");

        return match command {
            "get" => {
                if let Some(key) = args.next() {
                    Ok(Command::Get(Get::new(key.to_string())))
                } else {
                    Err(Error::CommandParse("failed parse empty key".to_string()))
                }
            },
            "set" => {
                if let (Some(key), Some(value)) = (args.next(), args.next()) {
                    Ok(Command::Set(Set::new(key.to_string(), Bytes::from(value.to_string()))))
                } else {
                    Err(Error::CommandParse("key,value,value type are required".to_string()))
                }
            },
            "delete" => {
                if let Some(key) = args.next() {
                    Ok(Command::Delete(Get::new(key.to_string())))
                } else {
                    Err(Error::CommandParse("failed parse empty key".to_string()))
                }
            }
            "all" => Ok(Command::All),
            "len" => Ok(Command::Len),
            "ping" => Ok(Command::Ping),
            _ => Err(Error::CommandParse("unsupported command".to_string()))

        };
    }

    pub fn from_frame(frame: Frame) -> Result<Command, CashError> {
        let array = match frame {
            Frame::Array(array) => array,
            _ => return Err(Error::Protocol(format!("protocol error; expected array, got {:?}", frame)))
        };

        let first = Command::command_frame(&array)?;
        let command = first.as_str();

        match command {
            "get" => Ok(Command::get(&array)?),
            "set" => Ok(Command::set(&array)?),
            "delete" => Ok(Command::delete(&array)?),
            "all" => Ok(Command::All),
            "len" => Ok(Command::Len),
            "ping" => Ok(Command::Ping),
            _ => {
                log::error!("unsupported command");
                return Err(Error::CommandParse("unsupported command".to_string()))
            }
        }
    }

    fn get(frames: &Vec<Frame>) -> Result<Command, CashError> {
        let key = Command::key_frame(frames)?;
        Ok(Command::Get(Get { key }))
    }

    fn set(frames: &Vec<Frame>) -> Result<Command, CashError> {
        let key = Command::key_frame(frames)?;
        let value = Command::value_frame(frames)?;
        Ok(Command::Set(Set { key, value }))
    }

    fn delete(frames: &Vec<Frame>) -> Result<Command, CashError> {
        let key = Command::key_frame(frames)?;
        Ok(Command::Delete(Get { key }))
    }

    fn command_frame(frames: &Vec<Frame>) -> Result<String, CashError> {
        if let Some(first) = frames.first() {
            match first {
                Frame::BulkString(command) => Ok(String::from_utf8(command.to_vec())?),
                _ => Err(Error::Protocol(format!("protocol error; expected command as bulk string, got {:?}", first)))
            }
        } else {
            Err(Error::CommandParse("empty command".to_string()))
        }
    }

    fn key_frame(frames: &Vec<Frame>) -> Result<String, CashError> {
        if let Some(second) = frames.get(1) {
            match second {
                Frame::BulkString(key) => Ok(String::from_utf8(key.to_vec())?),
                _ => Err(Error::Protocol(format!("protocol error; expected command as bulk string, got {:?}", second)))
            }
        } else {
            Err(Error::CommandParse("empty key".to_string()))
        }
    }

    fn value_frame(frames: &Vec<Frame>) -> Result<Bytes, CashError> {
        if let Some(third) = frames.get(2) {
            match third {
                Frame::BulkString(value) => Ok(value.clone()),
                _ => Err(Error::Protocol(format!("protocol error; expected value as bulk string, got {:?}", third)))
            }
        } else {
            Err(Error::CommandParse("empty value".to_string()))
        }
    }
}
