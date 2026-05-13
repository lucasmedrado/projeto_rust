use sqlx::PgPool;
use axum::{extract::{State}, Json, http::StatusCode};

use crate::validated_json::ValidatedJson;
use crate::dtos::user_login_dto::{LoginRequest, LoginResponse};
use crate::services::user_login_service::UserLoginService;
use crate::auth::jwt::generate_token;
use crate::errors::MyError;

pub async fn login_handler(
  State(pool): State<PgPool>,
  ValidatedJson(payload): ValidatedJson<LoginRequest>
) -> Result<(StatusCode, Json<LoginResponse>), MyError> {
  
  
  let user = UserLoginService::login_user(pool, payload.email).await?;
  
  let token = generate_token(user.id, &user.email)
  .map_err(|_e| MyError::DatabaseError(sqlx::Error::PoolClosed))?;    
  

  let response = LoginResponse {    
    token
  };

  Ok((StatusCode::CREATED, Json(response)))
}