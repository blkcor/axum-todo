use std::net::SocketAddr;

use axum::Router;
use axum_todo::handler::{backend, frontend};

#[tokio::main]
async fn main() {
    //tracing init
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "axum-todo=debug");
    }
    tracing_subscriber::fmt::init();
    //routers init
    let frentend_routers = frontend::router();
    let backend_routers = backend::router();
    let app = Router::new()
        .nest("/", frentend_routers)
        .nest("/admin", backend_routers);
    //service addr
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Listening on {}", addr);
    //start server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
