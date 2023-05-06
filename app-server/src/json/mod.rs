use async_trait::async_trait;
use bytes::Bytes;
use http_body_util::BodyExt;
use hyper::body::Incoming;
use hyper::Request;
use serde::{Deserialize, Serialize};
use crate::error::ServerError;

pub mod scheme;

#[async_trait]
pub trait Serializable<'a,'de, T: Serialize + Deserialize<'de>> {

    ///Сериализует байты в структуру которая реализует trait
    async fn serialize(bytes: &'de Bytes) -> Result<T, ServerError> {
        match serde_json::from_slice(bytes) {
            Ok(res) => Ok(res),
            Err(err) => Err(ServerError::SerdeJson(err.to_string()))
        }
    }

    ///Возвращает body запроса в байтах
    async fn bytes(req: Request<Incoming>) -> Result<Bytes, ServerError> {
        Ok(req.collect().await?.to_bytes())
    }

    async fn key(&'a self) -> String;
}