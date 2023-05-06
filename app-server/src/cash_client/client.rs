use bytes::Bytes;
use mini_casher::client::Client;
use mini_casher::SOCKET_ADDR;
use mini_casher::core::frames::Frame;
use crate::error::ServerError;

pub struct CashClient {
    connection: Client,
}

impl CashClient {
    pub async fn connect() -> Self {
        Self { connection: Client::connect(SOCKET_ADDR).await }
    }

    pub async fn get(&mut self, key: &str) -> Result<Option<String>, ServerError> {
        match self.connection.get(key).await? {
            Frame::BulkString(value) => Ok(Some(String::from_utf8(value.to_vec())?)),
            Frame::Null => Ok(None),
            _ => Err(ServerError::Cash("unexpected result".to_string()))
        }
    }

    pub async fn set(&mut self, key: &str, value: Bytes) -> Result<String, ServerError> {
        match self.connection.set(key, value).await? {
            Frame::Simple(str) => Ok(str),
            _ => Err(ServerError::Cash("unexpected result".to_string()))
        }
    }

    pub async fn delete(&mut self, key: &str) -> Result<Option<String>, ServerError> {
        match self.connection.delete(key).await? {
            Frame::BulkString(value) => Ok(Some(String::from_utf8(value.to_vec())?)),
            _ => Err(ServerError::Cash("unexpected result".to_string()))
        }
    }

    pub async fn all(&mut self) -> Result<Vec<Bytes>, ServerError> {
        match self.connection.all().await? {
            Frame::Array(arr) => {
                let mut json = vec![];

                for frame in arr {
                    match frame {
                        Frame::BulkString(value) => json.push(value),
                        _ => return Err(ServerError::Cash("unexpected result".to_string()))

                    }
                }
                Ok(json)
            },
            _ => Err(ServerError::Cash("unexpected result".to_string()))
        }
    }


}

async fn value_of_bulk(frame: &Frame) -> Result<String, ServerError> {
    match frame {
        Frame::BulkString(value) => Ok(String::from_utf8(value.to_vec())?),
        _ => Err(ServerError::Cash("unexpected result".to_string()))
    }
}




