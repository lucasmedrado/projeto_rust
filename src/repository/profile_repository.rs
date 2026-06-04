use uuid::Uuid;
use crate::errors::MyError;

use crate::models::profile::ProfileModel;
use crate::domain::profile::ProfileEntity;

pub struct ProfileRepository;

pub struct ProfileSummaryModel {
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
    full_name: Option<String>,
    localization: Option<String>,
    professional_objective: Option<String>,
	  slug: String,
    bio: Option<String>,	  
  ) -> Result<ProfileEntity, MyError>
    where
      for<'c> &'c mut E: sqlx::Executor<'c, Database = sqlx::Postgres>
  {      

    let profile = sqlx::query_as!(
      ProfileModel,    
      r#"INSERT INTO profiles (user_id, full_name, localization, professional_objective, slug, bio)
      VALUES ($1, $2, $3, $4, $5, $6)
      RETURNING id, user_id, full_name, localization, professional_objective, slug, bio, is_public 
      "#,
      user_id,
      full_name,
      localization,
      professional_objective,
      slug,
      bio
    )
    .fetch_one(executor)
    .await?;
    
    let profile_domain = ProfileEntity::from(profile); 
    
    Ok(profile_domain)
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


  pub async fn update_user<E>(
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
  ) -> Result<ProfileEntity, MyError>
    where 
       for<'c> &'c mut E: sqlx::Executor<'c, Database = sqlx::Postgres>
  {
    let user = sqlx::query_as!(    
      ProfileSummaryModel,  
      r#"
      SELECT 
        u.email,
        p.full_name, 
        p.localization, 
        p.professional_objective, 
        p.slug, 
        p.bio, 
        p.is_public 
      FROM profiles p 
      INNER JOIN users u 
      ON p.user_id = u.id
      WHERE p.user_id = $1     
      "#,
      user_id
    )
    .fetch_one(executor)
    .await?;
    
    let profile_entity = ProfileEntity::from(user);

    Ok(profile_entity)
  }

  
  pub async fn delete_profile_by_id<E>(
    &self,
    executor: &mut E,
    user_id: Uuid
  ) -> Result<ProfileEntity, MyError>
    where 
      for<'c> &'c mut E: sqlx::Executor<'c, Database = sqlx::Postgres> 
  {    

    let user = sqlx::query_as!(
      ProfileEntity,
      r#"
        DELETE FROM profiles
        WHERE user_id = $1
        RETURNING full_name, localization, professional_objective, slug, bio     
      "#,
      user_id
    )
    .fetch_one(executor)
    .await?;    

    Ok(user)
  }
  
}

