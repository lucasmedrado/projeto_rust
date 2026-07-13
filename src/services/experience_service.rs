use uuid::Uuid;
use sqlx::PgPool;

use crate::dtos::experience_dto::{CreateExperienceDto, validate_past_or_present, validate_period};
use crate::domain::experience::{ExperienceEntity};
use crate::repository::experience_repository::{ExperienceRepository, NewExperience};
//use crate::repository::profile_repository::ProfileRepository;
use crate::errors::MyError;


pub struct ExperienceService;

impl ExperienceService {
  pub async fn create_experience(pool: PgPool, user_id: Uuid, dto: CreateExperienceDto) -> Result<ExperienceEntity, MyError> {
    let mut tx = pool
    .begin()
    .await?;    

    let experience_repository = ExperienceRepository;    
    let new_experience = NewExperience::from(dto);
    let experience_domain = experience_repository.create_experience(&mut *tx , user_id, new_experience).await.map_err(|err| {
      println!("{err:?}");
      //return Err(err)
    }).unwrap();

    tx.commit().await?;

    Ok(experience_domain)
  }  
}