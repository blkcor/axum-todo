use axum::{routing::get, Router};

use super::auth;

pub mod index;

/// 前端路由
pub fn router() -> Router {
    Router::new()
        .route("/", get(index::index))
        .route("/auth", get(auth::login_ui).post(auth::login))
        .route("/logout", get(auth::logout))
}
