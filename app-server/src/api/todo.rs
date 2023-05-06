use hyper::body::Incoming;
use hyper::{Method, Request};
use crate::api::{not_found, preflight, ResponseResult};
use crate::service::todo;


pub async fn todo_api(req: Request<Incoming>) -> ResponseResult {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/todo/create") => todo::create(req).await,
        (&Method::OPTIONS, "/todo/create") => preflight(req).await,
        (&Method::GET, "/todo/get") => todo::get(req).await,
        (&Method::GET, "/todo/all") => todo::all(req).await,
        (&Method::OPTIONS, "/todo/delete") => preflight(req).await,
        (&Method::DELETE, "/todo/delete") => todo::delete(req).await,
        _ => not_found()
    }
}

