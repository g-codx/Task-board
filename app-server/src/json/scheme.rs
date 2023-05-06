use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::json::Serializable;

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    name: String,
    status: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Items {
    items: Vec<Item>
}

impl Items {
    pub fn new(items: Vec<Item>) -> Self {
        Self { items }
    }
}

#[async_trait]
impl <'a> Serializable<'a, '_, Item> for Item {
    async fn key(&'a self) -> String {
        self.name.clone()
    }
}
