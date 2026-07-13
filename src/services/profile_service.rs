use uuid::Uuid;
use sqlx::PgPool;

use crate::dtos::profile_dto::{CreateProfileDto};
use crate::domain::profile::{ProfileEntity};
use crate::repository::profile_repository::ProfileRepository;
use crate::errors::MyError;

pub struct ProfileService;

impl ProfileService {
  pub async fn create_profile(pool: PgPool, user_id: Uuid, dto: CreateProfileDto) -> Result<ProfileEntity, MyError> {
    let mut tx = pool
    .begin()
    .await?;    
    
    
    let user_id = ProfileService::find_by_id(pool, user_id).await.unwrap();

    let profile = ProfileRepository;
    let profile_domain = profile.create_profile(&mut *tx , user_id, dto.full_name, dto.localization, dto.professional_objective, dto.slug, dto.bio).await?;

    tx.commit().await?;

    Ok(profile_domain)
  }  


  pub async fn update_profile(pool: PgPool, user_id: Uuid, params: ProfileEntity) -> Result<ProfileEntity, MyError> {
    let mut tx = pool
    .begin()
    .await?;    

    let user_id = ProfileService::find_by_id(pool, user_id).await?;
    

    let profile = ProfileRepository;
    let profile_domain = profile.update_user(&mut *tx, user_id, params).await?;

    tx.commit().await?;

    Ok(profile_domain)
  }


  pub async fn find_profile_by_id(pool: PgPool, user_id: Uuid) -> Result<ProfileEntity, MyError> {
    let mut tx = pool
    .begin()
    .await?;

    let user_id = ProfileService::find_by_id(pool, user_id).await?;

    let profile = ProfileRepository;
    let profile_domain = profile.find_profile_by_id(&mut *tx, user_id).await?;

    tx.commit().await?;

    Ok(profile_domain)
  }


  pub async fn delete_profile_by_id(pool: PgPool, user_id: Uuid) -> Result<ProfileEntity, MyError> {
    let mut tx = pool
    .begin()
    .await?;    

    let profile = ProfileRepository;
    let profile_domain = profile.delete_profile_by_id(&mut *tx, user_id).await?;

    tx.commit().await?;

    Ok(profile_domain)
  }


  pub async fn find_by_id(pool: PgPool, id: Uuid) -> Result<Uuid, MyError> {
    let mut tx = pool
    .begin()
    .await?;

    let profile = ProfileRepository;
    let user_id = profile.find_by_id(&mut *tx, id).await?;
    
    tx.commit().await?;

    Ok(user_id)
  }

  
}