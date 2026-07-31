use uuid::Uuid;
use chrono::NaiveDate;

use crate::errors::MyError;
use crate::models::experience::ExperienceModel;
use crate::domain::experience::ExperienceEntity;
use crate::dtos::experience_dto::CreateExperienceDto;

pub struct NewExperience {  
  company_name: String,
  position: String,
  start_date: NaiveDate,
  end_date: Option<NaiveDate>,  
  description: Option<String>,
}

pub struct ProfileId {
  id: Uuid
}

impl From<CreateExperienceDto> for NewExperience {
  fn from(value: CreateExperienceDto) -> Self {
      Self {
        company_name: value.company_name,
        position: value.position,
        start_date: value.start_date,
        end_date: value.end_date,  
        description: value.description,
      }
  }
}

pub struct ExperienceRepository;

impl ExperienceRepository {

  pub async fn create_experience<E>(
    &self,
    executor: &mut E,
    user_id: Uuid,
    experience: NewExperience
  ) -> Result<ExperienceEntity, MyError>
    where 
      for<'c> &'c mut E: sqlx::Executor<'c, Database = sqlx::Postgres> 
  {
    let profile_id = self.fetch_id(executor, user_id).await?;
    
    let experience = sqlx::query_as!(
      ExperienceModel,
      r#"
        INSERT INTO experiences (profile_id, company_name, position, start_date, end_date, description) 
        VALUES ($1, $2, $3, $4, $5, $6) 
        RETURNING id, profile_id, company_name, position, start_date, end_date, description 
      "#,
      profile_id, 
      experience.company_name,
      experience.position,
      experience.start_date,
      experience.end_date,
      experience.description
    )
    .fetch_one(executor)
    .await?;

    let experience_domain: ExperienceEntity = experience.into();

    Ok(experience_domain)
  }

  pub async fn update_experience<E>(
    &self,
    executor: &mut E,
    user_id: Uuid,
    experience: NewExperience
  ) -> Result<ExperienceEntity, MyError>
    where 
      for<'c> &'c mut E: sqlx::Executor<'c, Database = sqlx::Postgres> 
    {
      let profile_id = self.fetch_id(executor, user_id).await?;

      let experience = sqlx::query_as!(
        ExperienceModel,
        r#"
          UPDATE experiences
          SET
            company_name = COALESCE($1, company_name), 
            position = COALESCE($2, position), 
            start_date = COALESCE($3, start_date), 
            end_date = COALESCE($4, end_date), 
            description = COALESCE($5, description)
          WHERE 
            profile_id = $6
          RETURNING
            id, profile_id, company_name, position, start_date, end_date, description 
        "#,
        experience.company_name,
        experience.position,
        experience.start_date,
        experience.end_date,
        experience.description,
        profile_id
      )
      .fetch_one(executor)
      .await?;

      let experience_domain: ExperienceEntity = experience.into();

      Ok(experience_domain)
    }

  pub async fn fetch_id<E>(
    &self,
    executor: &mut E,
    user_id: Uuid
  ) -> Result<Uuid, MyError>
    where 
       for<'c> &'c mut E: sqlx::Executor<'c, Database = sqlx::Postgres>
  {
    let user = sqlx::query_as!(
      ProfileId,        
      "SELECT id FROM profiles WHERE user_id = $1",
      user_id
    )
    .fetch_one(executor)
    .await?; 

    let profile_id = user.id;  

    Ok(profile_id)
  }

}