use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};

use serde_json::json;
use super::jwt::{validate_token, AuthError, Claims};

pub struct AuthUser(pub Claims);

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            AuthError::ExpiredToken  => (StatusCode::UNAUTHORIZED, "Token expirado"),
            AuthError::InvalidToken  => (StatusCode::UNAUTHORIZED, "Token inválido"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Erro interno"),
        };
        (status, Json(json!({ "error": msg }))).into_response()
    }
}

#[async_trait]
impl <S> FromRequestParts<S> for AuthUser 
  where S: Send + Sync 
  {
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
      let auth_header = parts
        .headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .ok_or(AuthError::InvalidToken)?;

      let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AuthError::InvalidToken)?;

      let claims = validate_token(token)?;

      Ok(AuthUser(claims))
    }
  }
