use axum::{http::StatusCode, response::{IntoResponse, Response}};
use sqlx::Error;
use crate::domain::user::UserEntity;
pub enum MyError {
    NotFound,
    DatabaseError(Error),
}

impl IntoResponse for MyError {
  fn into_response(self) -> Response {
      let (status, msg) = match self {
          MyError::NotFound => (StatusCode::NOT_FOUND, "Registro não encontrado"),
          MyError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Erro interno"),          
      };
      (status, msg).into_response()
  }
}

impl From<sqlx::Error> for MyError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            Error::RowNotFound => MyError::NotFound,
            Error::PoolClosed => MyError::DatabaseError(err),
            _ => MyError::DatabaseError(err)
        }
    }
}

