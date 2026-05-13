 use crate::models::profile::ProfileModel;
use crate::domain::profile::ProfileEntity;
//use crate::dtos::user_dto::UpdateUserDto;
use uuid::Uuid;
use crate::errors::MyError;

pub struct ProfileRepository;

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


  pub async fn find_user_by_id<E>(
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


  /*pub async fn find_user_by_email<E>(
    &self,
    executor: &mut E,
    email: &str
  ) -> Result<String, MyError>
    where 
       for<'c> &'c mut E: sqlx::Executor<'c, Database = sqlx::Postgres>
  {
    let user = sqlx::query_scalar!(      
      "SELECT email FROM users WHERE email = $1",
      email
    )
    .fetch_optional(executor)
    .await?;
    
    let exists = user.unwrap();

    Ok(exists)
  }

  pub async fn delete_user<E>(
    &self,
    executor: &mut E,
    email: &str
  ) -> Result<UserEntity, MyError>
    where 
      for<'c> &'c mut E: sqlx::Executor<'c, Database = sqlx::Postgres> 
  {
    let exists = self.user_exists(executor, &email).await?;

    if exists == false {
      return Err(MyError::NotFound);
    }

    let user = sqlx::query_as!(
      UserModel,
      r#"
        DELETE FROM users
        WHERE email = $1
        RETURNING id, email, password_hash      
      "#,
      email
    )
    .fetch_one(executor)
    .await?;

    let domain_user = UserEntity::from(user);

    Ok(domain_user)
  }

  pub async fn user_exists<E>(
    &self,
    executor: &mut E,
    email: &str
  ) -> Result<bool, MyError>
    where 
       for<'c> &'c mut E: sqlx::Executor<'c, Database = sqlx::Postgres>
  {
    let exists_user = sqlx::query_scalar!(
      r#"SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)"#,
      email
    )
    .fetch_one(executor)
    .await?;
    
    let exists = exists_user.unwrap();

    Ok(exists)
  }*/
}

