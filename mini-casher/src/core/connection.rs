use bytes::{Buf, BytesMut};
use std::io::Cursor;
use std::string::ToString;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::net::TcpStream;
use crate::core::error::CashError;
use crate::core::frames::Frame;


/**
Redis — это TCP-сервер, использующий модель клиент-сервер и так называемый протокол запроса/ответа.
Клиент подключается к серверу Redis, создавая TCP-соединение с портом 6379.
Redis принимает команды, состоящие из разных аргументов.
Как только команда получена, она обрабатывается, и ответ отправляется обратно клиенту.

`Connection` - структура которая обертывает `TcpStream` и считывает/записывает `Frame` значения.

При реализации сетевых протоколов сообщение по этому протоколу часто состоит
из нескольких меньших сообщений, известных как кадры.

Для чтения фреймов `Connection` использует внутренний буфер,который заполняется
до тех пор пока не будет достаточно байт для создания полного кадра.
Как только это происходит, `Connection` создает кадр и возвращает его вызывающей стороне.

При отправке кадров кадр сначала кодируется в буфер записи,затем содержимое буфера записи
записывается в сокет.

                        ***
Полное описание протокола в документации к redis:
- https://redis.io/docs/reference/protocol-spec/

Руководство по созданию framing от tokio:
- https://tokio.rs/tokio/tutorial/framing
 */
#[derive(Debug)]
pub struct Connection {
    pub socket: BufWriter<TcpStream>,
    pub buffer: BytesMut,
}

impl Connection {
    ///Создает обертку TcpStream и инициализирует buffer
    pub fn new(socket: TcpStream) -> Connection {
        Connection {
            socket: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(4096),
        }
    }

    ///Если для создания кадра достаточно данных метод вернет `Frame`
    ///При чтении из потока возращаемое значение 0, указывает на то,
    ///что данные больше не будут получены, если в буфере все ещё есть данные,
    ///значит получен неполный кадр и соединение разорвано.
    pub async fn read_frame(&mut self) -> Result<Option<Frame>, CashError> {
        loop {
            if let Some(frame) = self.parse_frame().await? {
                return Ok(Some(frame));
            }

            if self.socket.read_buf(&mut self.buffer).await? == 0 {
                return if self.buffer.is_empty() {
                    Ok(None)
                } else {
                    Err(CashError::SocketRead("connection reset by peer".to_string()))
                };
            }
        }
    }

    ///Записывает одно значение `Frame` в сокет.
    ///Вложенные массивы не поддерживаются
    pub async fn write_frame(&mut self, frame: &Frame) -> Result<(), CashError> {
        match frame {
            Frame::Array(val) => {
                let len = val.len() as u64;

                self.socket.write_u8(b'*').await?;
                self.write_u64(&len).await?;
                self.write_crlf().await?;
                for f in val {
                    self.write_value(f).await?;
                }
            }
            _ => {
                self.write_value(frame).await?;
            }
        }

        self.socket.flush().await?;
        Ok(())
    }

    ///При успешном преобразовании байтов из буффера создает `Frame`
    ///Если данных для создания `Frame` недостаточно
    /// возвращается `ERROR::Incomplete` и продолжается ожидание заполнения буффера
    ///При остальных возможных ошибках процесс преобразования прерывается
    async fn parse_frame(&mut self) -> Result<Option<Frame>, CashError> {
        let mut buff = Cursor::new(self.buffer.to_owned());
        let len = buff.position() as usize;
        buff.set_position(0);

        match Frame::try_frame(&mut buff) {
            Ok(frame) => {
                buff.advance(len);
                Ok(Some(frame))
            }
            Err(CashError::Incomplete) => Ok(None),
            Err(e) => Err(e.into())
        }
    }

    async fn write_value(&mut self, frame: &Frame) -> Result<(), CashError> {
        match frame {
            Frame::Simple(val) => {
                self.socket.write_u8(b'+').await?;
                self.socket.write_all(val.as_bytes()).await?;
                self.write_crlf().await?;
            }
            Frame::Error(val) => {
                self.socket.write_u8(b'-').await?;
                self.socket.write_all(val.as_bytes()).await?;
                self.write_crlf().await?;
            }
            Frame::Integer(val) => {
                self.socket.write_u8(b':').await?;
                self.write_u64(val).await?;
                self.write_crlf().await?;
            }
            Frame::Null => {
                self.socket.write_all(b"$-1\r\n").await?;
            }
            Frame::BulkString(val) => {
                let len = val.len() as u64;
                self.socket.write_u8(b'$').await?;
                self.write_u64(&len).await?;
                self.write_crlf().await?;
                self.socket.write_all(val).await?;
                self.write_crlf().await?;
            }
            Frame::Array(_) => {
                return Err(CashError::Protocol("nested arrays are not supported".to_string()));
            }
        }

        Ok(())
    }

    async fn write_crlf(&mut self) -> Result<(), CashError> {
        self.socket.write_all(b"\r\n").await?;
        Ok(())
    }

    async fn write_u64(&mut self, val: &u64) -> Result<(), CashError> {
        self.socket.write_all(val.to_string().as_bytes()).await?;
        Ok(())
    }
}