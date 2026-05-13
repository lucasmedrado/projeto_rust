use axum::{extract::{State, Path}, Json, http::StatusCode, response::IntoResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::validated_json::ValidatedJson;
use crate::dtos::profile_dto::{CreateProfileDto, UpdateProfileDto};
use crate::domain::profile::ProfileEntity;
use crate::services::profile_service::ProfileService; 
use crate::auth::middleware::AuthUser;
use crate::errors::MyError;

pub async fn create_profile_handler(
  AuthUser(claims): AuthUser,
  State(pool): State<PgPool>,
  ValidatedJson(payload): ValidatedJson<CreateProfileDto>
) -> Result<(StatusCode, Json<ProfileEntity>), MyError> {

  let user_id = claims.sub.parse::<Uuid>().unwrap();

  let profile = ProfileService::create_profile(pool, user_id, payload).await.unwrap();  
  
  Ok((StatusCode::CREATED, Json(profile)))
}

pub async fn update_profile_handler(
  AuthUser(claims): AuthUser,
  State(pool): State<PgPool>,
  ValidatedJson(payload): ValidatedJson<UpdateProfileDto>
) -> Result<(StatusCode, Json<ProfileEntity>), MyError> {
  
  let user_id = claims.sub.parse::<Uuid>().unwrap(); 
  
  let params: ProfileEntity = payload.into(); // Equivalente, maneira explícita -> let params = ProfileEntity::from(payload);

  let profile = ProfileService::update_profile(pool, user_id, params).await.unwrap();

  Ok((StatusCode::CREATED, Json(profile)))

}