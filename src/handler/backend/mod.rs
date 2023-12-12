use axum::{routing::get, Router};

use index::index;

pub mod index;

pub fn router() -> Router {
    Router::new().route("/", get(index))
}
