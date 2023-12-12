use axum::{routing::get, Router};

pub mod index;

/// 前端路由
pub fn router() -> Router {
    Router::new().route("/", get(index::index))
}
