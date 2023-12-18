//统一错误处理模块

use axum::{
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
};

#[derive(Debug)]
pub enum AppErrorType {
    NotFound,
    DB,
    Template,
    Dumplicate,
    Crypt,
    IncorrectLogin,
    Forbidden,
}

#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<Box<dyn std::error::Error>>,
    pub types: AppErrorType,
}

impl AppError {
    pub fn new(
        message: Option<String>,
        cause: Option<Box<dyn std::error::Error>>,
        types: AppErrorType,
    ) -> Self {
        Self {
            message,
            cause,
            types,
        }
    }

    pub fn from_err(cause: Box<dyn std::error::Error>, types: AppErrorType) -> Self {
        Self {
            message: None,
            cause: Some(cause),
            types,
        }
    }

    pub fn from_str(message: &str, types: AppErrorType) -> Self {
        Self::new(Some(message.to_string()), None, types)
    }

    pub fn notfound_opt(message: Option<String>) -> Self {
        Self::new(message, None, AppErrorType::NotFound)
    }

    pub fn notfound_str(message: &str) -> Self {
        Self::notfound_opt(Some(message.to_string()))
    }

    pub fn notfound() -> Self {
        Self::notfound_str("Not Found")
    }

    pub fn dumplicate(message: &str) -> Self {
        Self::from_str(message, AppErrorType::Dumplicate)
    }

    pub fn incorrect_login() -> Self {
        Self::from_str("错误的邮箱或密码", AppErrorType::IncorrectLogin)
    }
    pub fn forbidden() -> Self {
        Self::from_str("无权访问", AppErrorType::Forbidden)
    }
    pub fn response(self) -> axum::response::Response {
        match self.types {
            AppErrorType::Forbidden => {
                let mut hm = HeaderMap::new();
                hm.insert(header::LOCATION, "/auth".parse().unwrap());
                (StatusCode::FOUND, hm, ()).into_response()
            }
            _ => self
                .message
                .to_owned()
                .unwrap_or("有错误发生".to_string())
                .into_response(),
        }
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for AppError {}

//将第三方库的Error转换为AppError

/*
  deadpool_postgres::PoolError
*/
impl From<deadpool_postgres::PoolError> for AppError {
    fn from(value: deadpool_postgres::PoolError) -> Self {
        Self::from_err(Box::new(value), AppErrorType::DB)
    }
}

/*
tokio_postgres::Error
*/
impl From<tokio_postgres::Error> for AppError {
    fn from(value: tokio_postgres::Error) -> Self {
        Self::from_err(Box::new(value), AppErrorType::DB)
    }
}

/*
askama::Error
*/
impl From<askama::Error> for AppError {
    fn from(value: askama::Error) -> Self {
        Self::from_err(Box::new(value), AppErrorType::Template)
    }
}

///为了让AppError能作为axum的响应，需要实现 IntoResponse trait 在这里我们只需要响应错误信息即可
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let msg = match self.message {
            Some(message) => message.clone(),
            None => "有错误发生".to_string(),
        };
        msg.into_response()
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(err: bcrypt::BcryptError) -> Self {
        Self::from_err(Box::new(err), AppErrorType::Crypt)
    }
}
