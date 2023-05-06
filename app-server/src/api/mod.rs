use std::collections::HashMap;
use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
use hyper::{Request, Response, StatusCode};

mod todo;

pub const NOTFOUND: &[u8] = b"Not Found";
pub const BAD_REQUEST: &[u8] = b"Bad Request";
pub const INTERNAL_SERVER_ERROR: &[u8] = b"Internal Server Error";

pub type ResponseResult = Result<Response<BoxBody<Bytes, hyper::Error>>, Box<dyn std::error::Error + Send + Sync>>;


pub async fn router(req: Request<Incoming>) -> ResponseResult {
    match path(&req) {
        "todo" => todo::todo_api(req).await,
        _ => not_found()
    }
}

async fn preflight(req: Request<Incoming>) -> ResponseResult {
    dbg!("preflight: {}", &req);
    let _whole_body = req.collect().await?.aggregate();
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Headers", "*")
        .header("Access-Control-Allow-Methods", "POST, GET, OPTIONS, DELETE")
        .body(full(""))?;
    Ok(response)
}

pub fn params(req: &Request<Incoming>) -> HashMap<String, String> {
    req
        .uri()
        .query()
        .map(|v| {
            url::form_urlencoded::parse(v.as_bytes())
                .into_owned()
                .collect()
        })
        .unwrap_or_else(HashMap::new)
}

pub fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

pub fn response<T: Into<Bytes>>(chunk: T, status_code: StatusCode) -> ResponseResult {
    Ok(Response::builder()
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Headers", "*")
        .header("Access-Control-Allow-Methods", "POST, GET, OPTIONS")
        .status(status_code)
        .body(full(chunk)).unwrap())
}

pub fn empty_response() -> ResponseResult {
    response("", StatusCode::NO_CONTENT)
}

pub fn not_found() -> ResponseResult {
    response(NOTFOUND, StatusCode::NOT_FOUND)
}

pub fn bad_request() -> ResponseResult {
    response(BAD_REQUEST, StatusCode::BAD_REQUEST)
}

pub fn internal_server_error() -> ResponseResult {
    response(INTERNAL_SERVER_ERROR, StatusCode::INTERNAL_SERVER_ERROR)
}

fn path(req: &Request<Incoming>) -> &str {
    req
        .uri()
        .path()
        .split("/")
        .filter(|s| !s.is_empty())
        .next()
        .unwrap_or("")
}