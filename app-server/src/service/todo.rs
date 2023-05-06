use bytes::Bytes;
use hyper::{Request, StatusCode};
use hyper::body::Incoming;
use crate::api::{bad_request, not_found, params, response, ResponseResult};
use crate::json::scheme::{Item, Items};
use crate::json::Serializable;
use crate::cash_client::client::CashClient;


pub async fn create(req: Request<Incoming>) -> ResponseResult {
    let bytes = Item::bytes(req).await.unwrap();
    let json_scheme = Item::serialize(&bytes).await.unwrap();
    let json = serde_json::to_string(&json_scheme)?;
    let byte_json = Bytes::from(json.clone());

    let mut client = CashClient::connect().await;

    match client.set(json_scheme.key().await.as_str(), byte_json).await {
        Ok(value) => response(value, StatusCode::OK),
        Err(err) => response(err.to_string(), StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub async fn get(req: Request<Incoming>) -> ResponseResult {
    let params = params(&req);
    if let Some(key) = params.get("key") {
        let mut client = CashClient::connect().await;

        match client.get(key).await? {
            Some(json) => response(json, StatusCode::OK),
            None => not_found()
        }

    } else {
        bad_request()
    }
}

pub async fn all(_req: Request<Incoming>) -> ResponseResult {
    let mut client = CashClient::connect().await;
    let items = client.all().await?;

    let items: Vec<Item> = items
        .iter()
        .filter_map(|i| {
            let result = serde_json::from_slice::<Item>(i);

            if result.is_ok()  {
                Some(result.unwrap())
            } else {
                log::error!("{}", result.err().unwrap());
                None
            }

        })
        .collect();

    let json = serde_json::to_string(&Items::new(items))?;
    let res = response(json, StatusCode::OK);
    println!("{:?}",res);

    res
}

pub async fn delete(req: Request<Incoming>) -> ResponseResult {
    let params = params(&req);
    if let Some(key) = params.get("key") {
        let mut client = CashClient::connect().await;

        match client.delete(key).await? {
            Some(json) => response(json, StatusCode::OK),
            None => not_found()
        }

    } else {
        bad_request()
    }

}