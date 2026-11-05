use uuid::Uuid;
use crate::errors::MyError;

use crate::models::profile::ProfileModel;
use crate::domain::profile::ProfileEntity;

pub struct ProfileRepository;

pub struct ProfileSummaryModel {
    pub user_id: Uuid,
    pub email: Option<String>,
    pub full_name: Option<String>,
    pub localization: Option<String>,
    pub professional_objective: Option<String>,
    pub slug: Option<String>,
    pub bio: Option<String>,
    pub is_public: bool,
}

impl ProfileRepository {
  
  pub async fn create_profile<E>(
    &self, 
    executor: &mut E,
    user_id: Uuid, 
    params: ProfileEntity 
  ) -> Result<ProfileModel, MyError>
    where
      for<'c> &'c mut E: sqlx::Executor<'c, Database = sqlx::Postgres>
  {      

    let profile = sqlx::query_as!(
      ProfileModel,    
      r#"
      INSERT INTO profiles (user_id, full_name, localization, professional_objective, slug, bio)
      VALUES ($1, $2, $3, $4, $5, $6)
      RETURNING id, user_id, full_name, localization, professional_objective, slug, bio, is_public 
      "#,
      user_id,
      params.full_name,
      params.localization,
      params.professional_objective.unwrap(),
      params.slug,
      params.bio.unwrap()
    )
    .fetch_one(executor)
    .await?;     
    
    Ok(profile)
  }


  pub async fn find_by_id<E>(
    &self,
    executor: &mut E,        
    id: Uuid,
  ) -> Result<Uuid, MyError>
  where
    for<'c> &'c mut E: sqlx::Executor<'c, Database = sqlx::Postgres>,
  {
    let id = sqlx::query_scalar!(
        "SELECT id FROM users WHERE id = $1",
        id
    )
    .fetch_one(executor)
    .await?;

    Ok(id)
  }


  pub async fn update_profile<E>(
    &self,
    executor: &mut E,
    user_id: Uuid,    
    params: ProfileEntity
  ) -> Result<ProfileEntity, MyError>
    where 
      for<'c> &'c mut E: sqlx::Executor<'c, Database = sqlx::Postgres>
    {      

      let user = sqlx::query_as!(
        ProfileModel,
        r#"
        UPDATE profiles
        SET
          full_name = COALESCE($1, full_name),
          localization = COALESCE($2, localization),
          professional_objective = COALESCE($3, professional_objective),
          slug = COALESCE($4, slug),
          bio = COALESCE($5, bio)          
        WHERE user_id = $6
        RETURNING id, user_id, full_name, localization, professional_objective, slug, bio, is_public
        "#,
        params.full_name,
        params.localization,
        params.professional_objective,
        params.slug,
        params.bio,
        user_id       
      )
      .fetch_one(executor)
      .await?;

      let domain_profile = ProfileEntity::from(user);

      Ok(domain_profile)
  }  


  pub async fn find_profile_by_id<E>(
    &self,
    executor: &mut E,
    user_id: Uuid
  ) -> Result<ProfileModel, MyError>
    where 
       for<'c> &'c mut E: sqlx::Executor<'c, Database = sqlx::Postgres>
  {
    let user = sqlx::query_as!(    
      ProfileModel,  
      r#"
      SELECT 
        id, 
        user_id, 
        full_name, 
        localization, 
        professional_objective, 
        slug, 
        bio, 
        is_public        
      FROM 
        profiles      
      WHERE 
        user_id = $1     
      "#,
      user_id
    )
    .fetch_one(executor)
    .await?;    

    Ok(user)

  }

  
  pub async fn delete_profile_by_id<E>(
    &self,
    executor: &mut E,
    user_id: Uuid
  ) -> Result<ProfileModel, MyError>
    where 
      for<'c> &'c mut E: sqlx::Executor<'c, Database = sqlx::Postgres> 
  {    

    let user = sqlx::query_as!(
      ProfileModel,
      r#"
        DELETE FROM profiles
        WHERE user_id = $1
        RETURNING id, user_id, full_name, localization, professional_objective, slug, bio, is_public     
      "#,
      user_id
    )
    .fetch_one(executor)
    .await?;    

    Ok(user)
  }
  
}

