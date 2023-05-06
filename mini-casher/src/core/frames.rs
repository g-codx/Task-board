use std::io::Cursor;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use crate::core::error::CashError;

/**
RESP является протоколом сериализации, который поддерживает следующие типы данных:
 - простые строки (Simple Strings)
 - ошибки (Errors)
 - целые числа (Integers)
 - объемные строки (Bulk Strings)
 - массивы (Arrays)

RESP может представлять значение Null, используя специальную вариацию Bulk Strings или Array

В RESP первый байт определяет тип данных:

 - для простых строк первым байтом ответа является «+».
 - для Errors первый байт ответа равен "-"
 - для целых чисел первым байтом ответа является ":"
 - для Bulk Strings первым байтом ответа является "$"
 - для массивов первым байтом ответа является " *"


Кадрирование — это процесс получения потока байтов и преобразования его в поток кадров.
Фрейм — это единица данных, передаваемая между двумя одноранговыми узлами.

                        ***
Полное описание протокола в документации к redis:

- https://redis.io/docs/reference/protocol-spec/

Руководство по созданию framing от tokio:

- https://tokio.rs/tokio/tutorial/framing
 */
#[derive(Clone, Debug, PartialEq)]
pub enum Frame {
    Simple(String),
    Error(String),
    Integer(u64),
    BulkString(Bytes),
    Null,
    Array(Vec<Frame>),
}

impl Frame {
    ///Реализует преобразование байтов из буффера по спецификации протокола
    ///При успешном преобразовании создаёт `Frame` из байтов в буффере
    ///Возвращает ошибки типа `ERROR::Incomplete` и `ERROR::Protocol`
    pub fn try_frame(buff: &mut Cursor<BytesMut>) -> Result<Frame, CashError> {
        match pop(buff)? {
            b'+' => simple_string_frame(buff),
            b'-' => error_frame(buff),
            b':' => decimal_frame(buff),
            b'$' => bulk_string_frame(buff),
            b'*' => array_frame(buff),
            unknown => Err(CashError::Protocol(
                format!("protocol error; invalid frame type byte `{}`", unknown.to_string())))
        }
    }
}

///При успешном пробразовании возвращает - `Frame::Simple`
///Возможные ошибки:
/// `ERROR:Incomplete` при неполных данных
/// `ERORR::Protocol` при неудачном преобразовании байтов в string
fn simple_string_frame(buff: &mut Cursor<BytesMut>) -> Result<Frame, CashError> {
    let line = line(buff)?.to_vec();
    let string = String::from_utf8(line)?;
    Ok(Frame::Simple(string))
}

///При успешном преобразовании возвращает - `Frame::Error`
///Возможные ошибки:
/// `ERROR:Incomplete` при неполных данных
/// `ERORR::Protocol` при неудачном преобразовании байтов в string
fn error_frame(buff: &mut Cursor<BytesMut>) -> Result<Frame, CashError> {
    let line = line(buff)?.to_vec();
    let string = String::from_utf8(line)?;
    Ok(Frame::Error(string))
}

///При успешном преобразовании возвращает - `Frame::Integer`
///Возможные ошибки:
/// `ERROR:Incomplete` при неполных данных
/// `ERORR::Protocol` при неудачном преобразовании байтов в u64
fn decimal_frame(buff: &mut Cursor<BytesMut>) -> Result<Frame, CashError> {
    let line = line(buff)?.to_vec();
    let decimal = decimal(line)?;
    Ok(Frame::Integer(decimal))
}

///При успешном преобразовании возвращает:
/// `Frame::BulkString(data)`
/// `Frame::Null`
///Возможные ошибки:
/// `ERROR:Incomplete` при неполных данных
/// `ERORR::Protocol` при нарешении формы протокола
fn bulk_string_frame(buff: &mut Cursor<BytesMut>) -> Result<Frame, CashError> {
    return match peek(buff)? {
        b'-' => {
            let line = line(buff)?;

            if b"-1" != line {
                return Err(CashError::Protocol("protocol error; invalid frame format".to_string()))
            }

            Ok(Frame::Null)
        },
        _ => {
            let line = line(buff)?.to_vec();
            let len = decimal(line)? as usize;
            let n = len + 2;

            if buff.remaining() < n {
                return Err(CashError::Incomplete)
            }

            let data = Bytes::copy_from_slice(&buff.chunk()[..len]);
            buff.advance(len);

            if !skip_crlf(buff)? {
                return Err(CashError::Incomplete)
            }

            Ok(Frame::BulkString(data))
        }
    }
}

///При успешном преобразовании возвращает -`Frame::Array(Vec<Frame>)`
///Возвращаемые ошибки зависят от типов `Frame`
fn array_frame(buff: &mut Cursor<BytesMut>) -> Result<Frame, CashError> {
    let line = line(buff)?.to_vec();
    let len = decimal(line)? as usize;
    let mut arr = Vec::with_capacity(len);

    for _ in 0..len {
        arr.push(Frame::try_frame(buff)?);
    }

    Ok(Frame::Array(arr))
}

///Возвращает первый байт как u8, позиция увеличиватеся на 1.
///Если в буффере нет данных - `ERROR:Incomplete`
fn pop(buff: &mut Cursor<BytesMut>) -> Result<u8, CashError> {
    if !buff.has_remaining() {
        return Err(CashError::Incomplete);
    }

    return Ok(buff.get_u8());
}

///Возвращает первый байт как u8, позиция не изменяется.
///Если в буффере нет данных - `ERROR:Incomplete`
fn peek(buff: &mut Cursor<BytesMut>) -> Result<u8, CashError> {
    if !buff.has_remaining() {
        return Err(CashError::Incomplete);
    }

    return Ok(buff.chunk()[0] as u8);
}

///Вовращает последовательность байтов от текущей позиции до CRLF.
///Позиция увеличивается на всю пройденную длину включая CRLF.
///Если в конце буффера нет CRLF - `ERROR:Incomplete`
fn line(buff: &mut Cursor<BytesMut>) -> Result<&[u8], CashError> {
    let start = buff.position() as usize;
    let end = buff.get_ref().len() - 1;

    for i in start..end {
        if next_crlf(buff.get_ref()[i], buff.get_ref()[i + 1]) {
            buff.set_position((i + 2) as u64);
            return Ok(&buff.get_ref()[start..i]);
        }
    }

    Err(CashError::Incomplete)
}

///Возвращает u64 при успешном преобразовании.
///Для преобразования байтов в целочисленный тип используется `atoi`.
///При ошибке преобразования, вернет ошибку протокола - `ERROR:Protocol`
fn decimal(line: Vec<u8>) -> Result<u64, CashError> {
    use atoi::atoi;
    match atoi::<u64>(&line) {
        Some(decimal) => Ok(decimal),
        None => Err(CashError::Protocol("protocol error; invalid decimal frame format".to_string()))
    }
}

///Перемещает позицию на 2, если слудующие байты это - crlf(\r\n)
fn skip_crlf(buff: &mut Cursor<BytesMut>) -> Result<bool, CashError> {
    if buff.remaining() < 2 {
        return Err(CashError::Incomplete);
    }

    let position = buff.position() as usize;

    for i in position..position + 1 {
        if next_crlf(buff.get_ref()[i], buff.get_ref()[i + 1]) {
            buff.set_position((position + 2) as u64);
            return Ok(true);
        }
    }

    return Ok(false);
}


fn next_crlf(current: u8, next: u8) -> bool {
    current == b'\r' && next == b'\n'
}


mod frames_tests {
    use super::*;

    #[tokio::test]
    async fn try_frame_simple_string_ok() {
        let mut buff = test_data(&b"+hello frame\r\n"[..]);

        let string_line = Frame::try_frame(&mut buff);
        assert_eq!(Ok(Frame::Simple("hello frame".to_string())), string_line);
    }

    #[tokio::test]
    async fn try_frame_simple_string_err() {
        let mut buff = test_data(&b"+hello "[..]);

        let string_line = Frame::try_frame(&mut buff);
        assert_eq!(Err(CashError::Incomplete), string_line);
    }

    #[tokio::test]
    async fn try_frame_decimal_ok() {
        let mut buff = test_data(&b":1984\r\n"[..]);

        let decimal = Frame::try_frame(&mut buff);
        assert_eq!(Ok(Frame::Integer(1984)), decimal)
    }

    #[tokio::test]
    async fn try_frame_decimal_err() {
        let mut buff = test_data(&b":19"[..]);

        let decimal_err = Frame::try_frame(&mut buff);
        assert_eq!(Err(CashError::Incomplete), decimal_err);
    }

    #[tokio::test]
    async fn try_frame_decimal_unexpected_value() {
        let mut buff = test_data(&b":hello\r\n"[..]);

        let decimal_err = Frame::try_frame(&mut buff);
        assert_eq!(Err(CashError::Protocol(
            "protocol error; invalid decimal frame format".to_string())), decimal_err);
    }

    #[tokio::test]
    async fn try_frame_error_ok() {
        let mut buff = test_data(&b"-error message\r\n"[..]);

        let error_line = Frame::try_frame(&mut buff);
        assert_eq!(Ok(Frame::Error("error message".to_string())), error_line);
    }

    #[tokio::test]
    async fn try_frame_error_err() {
        let mut buff = test_data(&b"-er"[..]);

        let error_line = Frame::try_frame(&mut buff);
        assert_eq!(Err(CashError::Incomplete), error_line);
    }

    #[tokio::test]
    async fn try_frame_bulk_ok_incomplete_test() {
        let mut buff = test_data(&b"$5\r\nhell"[..]);

        let frame = Frame::try_frame(&mut buff);
        assert_eq!(Err(CashError::Incomplete), frame);
    }

    #[tokio::test]
    async fn try_frame_bulk_ok_test() {
        let mut buff = test_data(&b"$5\r\nhello\r\n"[..]);

        let frame = Frame::try_frame(&mut buff);
        assert_eq!(Ok(Frame::BulkString(Bytes::from("hello"))), frame);
    }

    #[tokio::test]
    async fn try_frame_bulk_err_test() {
        let mut buff = test_data(&b"$-2\r\n"[..]);

        let frame = Frame::try_frame(&mut buff);
        assert_eq!(Err(CashError::Protocol("protocol error; invalid frame format".to_string())), frame);
    }

    #[tokio::test]
    async fn try_frame_bulk_null_test() {
        let mut buff = test_data(&b"$-1\r\n"[..]);

        let frame = Frame::try_frame(&mut buff);
        assert_eq!(Ok(Frame::Null), frame);
    }

    #[tokio::test]
    async fn try_frame_bulk_incomplete_test() {
        let mut buff = test_data(&b"$-1"[..]);

        let frame = Frame::try_frame(&mut buff);
        assert_eq!(Err(CashError::Incomplete), frame);
    }

    #[tokio::test]
    async fn try_frame_empty_buff() {
        let mut empty_buff = test_data(&b""[..]);

        let empty = Frame::try_frame(&mut empty_buff);
        assert_eq!(Err(CashError::Incomplete), empty);
    }

    #[tokio::test]
    async fn try_frame_not_protocol_err() {
        let mut not_protocol_buff = test_data(&b"hello frame"[..]);

        let not_protocol_line = Frame::try_frame(&mut not_protocol_buff);
        assert_eq!(Err(CashError::Protocol(
            format!("protocol error; invalid frame type byte `{}`", b'h'))), not_protocol_line);
    }

    #[tokio::test]
    async fn try_frame_arr_ok_test() {
        let mut buff = test_data(
            &b"*4\r\n$5\r\nhello\r\n$5\r\nworld\r\n:111\r\n-Error message\r\n"[..]);

        let frame = Frame::try_frame(&mut buff);
        let expected = Frame::Array(vec![
            Frame::BulkString(Bytes::from("hello")),
            Frame::BulkString(Bytes::from("world")),
            Frame::Integer(111),
            Frame::Error("Error message".to_string())
        ]);

        assert_eq!(Ok(expected), frame);
    }

    #[tokio::test]
    async fn try_frame_arr_incomplete_test() {
        let mut buff = test_data(&b"*1\r\n$5\r\nhello\r\n$5\r\nworld\r\n"[..]);

        let frame = Frame::try_frame(&mut buff);
        println!("{:?}" , frame);
    }

    #[tokio::test]
    async fn try_frame_arr_len_test() {
        let mut buff = test_data(&b"*3\r\n$5\r\nhello\r\n"[..]);

        let frame = Frame::try_frame(&mut buff);
        assert_eq!(Err(CashError::Incomplete), frame);
    }

    #[tokio::test]
    async fn skip_crlf_test_ok() {
        let mut buff = test_data(&b"$\r\n"[..]);
        let first = pop(&mut buff);
        assert_eq!(Ok(b'$'), first);

        assert_eq!(Ok(true), skip_crlf(&mut buff));
        assert_eq!(3, buff.position());
    }

    #[tokio::test]
    async fn skip_crlf_test_err() {
        let mut buff = test_data(&b"$"[..]);
        let first = pop(&mut buff);
        assert_eq!(Ok(b'$'), first);

        assert_eq!(Err(CashError::Incomplete), skip_crlf(&mut buff));
    }

    #[tokio::test]
    async fn get_first_ok_test() {
        let mut buff = test_data(&b"+hello\r\n"[..]);

        let first = pop(&mut buff);
        assert_eq!(Ok(b'+'), first);
    }

    #[tokio::test]
    async fn get_first_err_test() {
        let mut buff = test_data(&b""[..]);

        let first = pop(&mut buff);
        assert_eq!(Err(CashError::Incomplete), first);
    }

    #[tokio::test]
    async fn get_line_ok_test() {
        let mut buff = test_data(&b"+hello\r\n"[..]);

        let first = pop(&mut buff);
        assert_eq!(Ok(b'+'), first);

        let line = line(&mut buff);
        assert_eq!(Ok("hello".as_bytes()), line);
    }

    #[tokio::test]
    async fn get_line_err_test() {
        let mut buff = test_data(&b"+hel"[..]);

        let first = pop(&mut buff);
        assert_eq!(Ok(b'+'), first);

        let line = line(&mut buff);
        assert_eq!(Err(CashError::Incomplete), line);
    }


    fn test_data(bytes: &[u8]) -> Cursor<BytesMut> {
        let mut input = BytesMut::with_capacity(10);
        input.put(bytes);

        Cursor::new(input)
    }
}

