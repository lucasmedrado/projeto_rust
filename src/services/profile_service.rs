use uuid::Uuid;
use sqlx::PgPool;

use crate::dtos::profile_dto::{CreateProfileDto};
use crate::domain::profile::{ProfileEntity};
use crate::repository::profile_repository::ProfileRepository;
use crate::errors::MyError;
use crate::repository::user_repository::UserRepository;

pub struct ProfileService;

impl ProfileService { 
  pub async fn create_profile(pool: PgPool, user_id: Uuid, dto: CreateProfileDto) -> Result<ProfileEntity, MyError> {
    let mut tx = pool
    .begin()
    .await?;    

    let user_repository = UserRepository;
     
    let profile_model = user_repository.find_user_by_id(&mut *tx, &user_id).await?;
    
    match user_id != profile_model.id.unwrap() {
      true => return Err(MyError::NotFound),
      false => (),
    }    

    let profile = ProfileRepository;
    let dto = ProfileEntity::from(dto);
    let profile_model = profile.create_profile(&mut *tx , user_id, dto).await?;
    
    let profile_domain = ProfileEntity::from(profile_model);

    tx.commit().await?;

    Ok(profile_domain)
  }  


  pub async fn update_profile(pool: PgPool, user_id: Uuid, params: ProfileEntity) -> Result<ProfileEntity, MyError> {
    let mut tx = pool
    .begin()
    .await?;    

    let profile_repository = ProfileRepository;
     
    let profile_model = profile_repository.find_profile_by_id(&mut *tx, user_id).await?;

    match user_id != profile_model.user_id {
      true => return Err(MyError::NotFound),
      false => (),
    }    

    let profile = ProfileRepository;
    let profile_model = profile.update_profile(&mut *tx , profile_model.user_id, params).await?;

    let profile_domain = ProfileEntity::from(profile_model);

    tx.commit().await?;

    Ok(profile_domain)
  }


  pub async fn find_profile_by_id(pool: PgPool, user_id: Uuid) -> Result<ProfileEntity, MyError> {
    let mut tx = pool
    .begin()
    .await?;

    let profile_repository = ProfileRepository;
     
    let profile_model = profile_repository.find_profile_by_id(&mut *tx, user_id).await?;

    match user_id != profile_model.user_id {
      true => return Err(MyError::NotFound),
      false => (),
    }    

    let profile = ProfileRepository;
    let profile_model = profile.find_profile_by_id(&mut *tx , profile_model.user_id).await?;

    let profile_domain = ProfileEntity::from(profile_model);

    tx.commit().await?;

    Ok(profile_domain)
  }


  pub async fn delete_profile_by_id(pool: PgPool, user_id: Uuid) -> Result<ProfileEntity, MyError> {
    let mut tx = pool
    .begin()
    .await?;    

    /*let profile = ProfileRepository;
    let profile_domain = profile.delete_profile_by_id(&mut *tx, user_id).await?;

    tx.commit().await?;

    Ok(profile_domain)*/

    let profile_repository = ProfileRepository;
     
    let profile_model = profile_repository.find_profile_by_id(&mut *tx, user_id).await?;

    match user_id != profile_model.user_id {
      true => return Err(MyError::NotFound),
      false => (),
    }    

    let profile = ProfileRepository;
    let profile_model = profile.delete_profile_by_id(&mut *tx , profile_model.user_id).await?;

    let profile_domain = ProfileEntity::from(profile_model);

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