pub mod db;
pub mod error;
pub mod handler;
pub mod view;

/*
  自定义Result
*/

pub type Result<T> = std::result::Result<T, error::AppError>;
