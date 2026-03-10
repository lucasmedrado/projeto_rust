use axum::extract::Query;
use axum::{extract::{State, Path}, Json, http::StatusCode, response::IntoResponse};
use sqlx::PgPool;


use crate::validated_json::ValidatedJson;
use crate::dtos::user_dto::{CreateUserDto, EmailQueryDto, UpdateUserDto};
use crate::domain::user::UserEntity;
use crate::services::user_service::UserService; 
use crate::errors::MyError;

pub async fn create_user_handler(
  State(pool): State<PgPool>,
  ValidatedJson(payload): ValidatedJson<CreateUserDto>
) -> Result<(StatusCode, Json<UserEntity>), MyError> {
  
  let user = UserService::create_user(pool, payload).await?;

  Ok((StatusCode::CREATED, Json(user)))
}


pub async fn update_user_handler(  
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