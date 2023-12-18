use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::{extractor_middleware, Extension};
use axum::Router;
use axum_todo::config::Config;
use axum_todo::handler::{backend, frontend};
use axum_todo::{middleware, AppState};

#[tokio::main]
async fn main() {
    //config init
    dotenv::dotenv().ok();
    let cfg = Config::from_env().expect("配置初始化失败");
    let pool = cfg.pg.create_pool(None, tokio_postgres::NoTls).unwrap();
    //tracing init
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "axum_todo=debug");
    }
    tracing_subscriber::fmt::init();
    //routers init
    let frentend_routers = frontend::router();
    let backend_routers = backend::router().layer(extractor_middleware::<middleware::Auth>());
    let app = Router::new()
        .nest("/", frentend_routers)
        .nest("/admin", backend_routers)
        .layer(Extension(Arc::new(AppState { pool })));
    //service addr
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Listening on {}", addr);
    //start server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
