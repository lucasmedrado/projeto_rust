use axum::extract::Query;
use axum::{extract::{State, Path}, Json, http::StatusCode, response::IntoResponse};
use sqlx::PgPool;


use crate::validated_json::ValidatedJson;
use crate::dtos::user_dto::{CreateUserDto, EmailQueryDto, UpdateUserDto, UserAuthedResponse};
use crate::domain::user::UserEntity;
use crate::services::user_service::UserService; 
use crate::auth::middleware::AuthUser;
use crate::auth::jwt::generate_token;
use crate::errors::MyError;

pub async fn create_user_handler(
  State(pool): State<PgPool>,
  ValidatedJson(payload): ValidatedJson<CreateUserDto>
) -> Result<(StatusCode, Json<UserAuthedResponse>), MyError> {
  
  let user = UserService::create_user(pool, payload).await?;
  let id = user.id.unwrap();
  let email = user.email.as_str();
  println!("controller");
  let token = generate_token(id, &email)
  .map_err(|_e| {
    println!("{_e}");
    MyError::DatabaseError(sqlx::Error::PoolClosed)
  })?;

  let response = UserAuthedResponse {
    user,
    token,
  };

  Ok((StatusCode::CREATED, Json(response)))
}

pub async fn update_user_handler(  
  AuthUser(claims): AuthUser,
  Path(email): Path<String>,
  State(pool): State<PgPool>,
  ValidatedJson(payload): ValidatedJson<UpdateUserDto>
) -> Result<(StatusCode, Json<UserEntity>), MyError> {  

  let user = UserService::update_user(pool, email, payload).await?;

  Ok((StatusCode::OK, Json(user)))
}

pub async fn find_user_handler(
  State(pool): State<PgPool>,
  Query(params): Query<EmailQueryDto>
) -> Result<impl IntoResponse, MyError> {

  let user = UserService::find_user(pool, &params.email).await?;

  Ok((StatusCode::FOUND, Json(user)))
}

pub async fn delete_user_handler(
  Path(email): Path<String>,
  State(pool): State<PgPool>
) -> Result<(StatusCode, Json<UserEntity>), MyError> {

  let user = UserService::delete_user(pool, &email).await?;

  Ok((StatusCode::FOUND, Json(user)))
}