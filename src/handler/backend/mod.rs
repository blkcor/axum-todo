use axum::{routing::get, Router};

use index::index;
use serde::Deserialize;

pub mod category;
pub mod index;

pub fn router() -> Router {
    Router::new().route("/", get(index))
}

#[derive(Deserialize)]
pub struct Args {
    pub msg: Option<String>,
    pub page: Option<u32>,
}
impl Args {
    pub fn msg(&self) -> String {
        self.msg.clone().unwrap_or("".to_string())
    }
    pub fn page(&self) -> u32 {
        self.page.unwrap_or(0)
    }
}
