use axum::{extract::{State, Path}, Json, http::StatusCode, response::IntoResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::validated_json::ValidatedJson;
use crate::dtos::experience_dto::{CreateExperienceDto};
use crate::domain::experience::ExperienceEntity;
use crate::services::experience_service::ExperienceService; 
use crate::auth::middleware::AuthUser;
use crate::errors::MyError;

pub async fn create_experience_handler(
  AuthUser(claims): AuthUser,
  State(pool): State<PgPool>,
  ValidatedJson(payload): ValidatedJson<CreateExperienceDto>
) -> Result<(StatusCode, Json<ExperienceEntity>), MyError> {

  let user_id = claims.sub.parse::<Uuid>().unwrap();  
  let experience = ExperienceService::create_experience(pool, user_id, payload).await?;  
  
  Ok((StatusCode::CREATED, Json(experience)))
}

pub async fn update_experience_handler(
  AuthUser(claims): AuthUser,
  State(pool): State<PgPool>,
  ValidatedJson(payload): ValidatedJson<CreateExperienceDto>
) -> Result<(StatusCode, Json<ExperienceEntity>), MyError> {

  let user_id = claims.sub.parse::<Uuid>().unwrap();  
  let experience = ExperienceService::update_experience(pool, user_id, payload).await?;  
  
  Ok((StatusCode::CREATED, Json(experience)))
}


pub async fn find_experience_handler(
  AuthUser(claims): AuthUser,
  State(pool): State<PgPool>,  
) -> Result<(StatusCode, Json<ExperienceEntity>), MyError> {

  let user_id = claims.sub.parse::<Uuid>().unwrap();  
  let experience = ExperienceService::find_experience(pool, user_id).await?;  
  
  Ok((StatusCode::CREATED, Json(experience)))
}