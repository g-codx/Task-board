use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use crate::{SOCKET_ADDR, Storage};
use crate::core::command::Command;
use crate::core::connection::Connection;
use crate::core::error::{CashError};
use crate::core::frames::Frame;

pub async fn run() {
    let listener = TcpListener::bind(SOCKET_ADDR).await.unwrap();
    let storage = Arc::new(Mutex::new(HashMap::new()));

    log::info!("Listening: {}", SOCKET_ADDR);

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let storage = storage.clone();

        tokio::spawn(async move {
            handler(socket, storage).await
        });
    }
}

async fn handler(socket: TcpStream, storage: Storage) -> Result<(), CashError> {
    let mut connection = Connection::new(socket);

    let command = read_command(&mut connection).await?;

    if let Some(command) = command {
        let response = execute(command, storage).await?;
        connection.write_frame(&response).await?;
    }

    Ok(())
}

async fn read_command(connection: &mut Connection) -> Result<Option<Command>, CashError> {
    let some_frame = match connection.read_frame().await {
        Ok(some_frame) => some_frame,
        Err(err) => {
            connection.write_frame(&Frame::Error(err.to_string())).await?;
            log::error!("{}", err);
            None
        }
    };

    if let Some(frame) = some_frame {
        let command = Command::from_frame(frame)?;
        Ok(Some(command))
    } else {
        Ok(None)
    }
}

async fn execute(command: Command, storage: Storage) -> Result<Frame, CashError> {
    match command {
        Command::Get(get) => {
            let storage = storage.lock()?;
            if let Some(value) = storage.get(get.key().as_str()) {
                Ok(Frame::BulkString(value.clone()))
            } else {
                Ok(Frame::Null)
            }
        }
        Command::Set(set) => {
            let mut storage = storage.lock()?;
            storage.insert(set.key().clone(), set.value().clone());
            Ok(Frame::Simple("Ok".to_string()))
        }
        Command::Delete(delete) => {
            let mut storage = storage.lock()?;
            storage.remove(delete.key()).ok_or(CashError::Storage("remove failed".to_string()))?;
            Ok(Frame::Simple("Ok".to_string()))
        }
        Command::Len => {
            let len = storage.lock()?.len() as u64;
            Ok(Frame::Integer(len))
        }
        Command::All => {
            let all: Vec<Frame> = storage
                .lock()
                .unwrap()
                .values()
                .cloned()
                .map(|b| Frame::BulkString(b.clone()))
                .collect();

            Ok(Frame::Array(all))
        }
        Command::Ping => Ok(Frame::Simple("PONG".to_string()))
    }
}