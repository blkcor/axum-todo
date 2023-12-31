pub mod config;
pub mod cookie;
pub mod db;
pub mod error;
pub mod form;
pub mod handler;
pub mod md;
pub mod middleware;
pub mod model;
pub mod password;
pub mod view;

/*
  自定义Result
*/

pub type Result<T> = std::result::Result<T, error::AppError>;

pub struct AppState {
    /// 数据库连接
    pub pool: deadpool_postgres::Pool,
}
