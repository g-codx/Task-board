use bytes::Bytes;
use tokio::net::TcpStream;
use crate::core::command::Command;
use crate::core::connection::Connection;
use crate::core::error::CashError;

use crate::core::frames::Frame;


pub struct Client {
    connection: Connection,
}

impl Client {
    pub async fn connect(addr: &str) -> Self {
        match TcpStream::connect(addr).await {
            Ok(socket) => Self { connection: Connection::new(socket) },
            Err(e) => panic!("Failed connection: {:?}", e)
        }
    }

    pub async fn get(&mut self, key: &str) -> Result<Frame, CashError> {
        let frame = Command::get_frame(key);
        self.execute(&frame).await
    }

    pub async fn set(&mut self, key: &str, value: Bytes) -> Result<Frame, CashError> {
        let frame = Command::set_frame(key, value);
        self.execute(&frame).await
    }

    pub async fn delete(&mut self, key: &str) -> Result<Frame, CashError> {
        let frame = Command::delete_frame(key);
        self.execute(&frame).await
    }

    pub async fn len(&mut self) -> Result<Frame, CashError> {
        let frame = Command::len_frame();
        self.execute(&frame).await
    }

    pub async fn ping(&mut self) -> Result<Frame, CashError> {
        let frame = Command::ping_frame();
        self.execute(&frame).await
    }

    pub async fn all(&mut self) -> Result<Frame, CashError> {
        let frame = Command::all_frame();
        self.execute(&frame).await
    }

    async fn execute(&mut self, frame: &Frame) -> Result<Frame, CashError> {
        self.connection.write_frame(frame).await?;
        let response = self.connection.read_frame().await;

        match response {
            Ok(some_frame) => {
                if let Some(frame) = some_frame {
                    Ok(frame)
                } else {
                    Ok(Frame::Simple("empty response".to_string()))
                }
            }
            Err(err) => Err(err)
        }
    }
}
