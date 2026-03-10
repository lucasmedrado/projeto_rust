use crate::models::user::UserModel;
use crate::domain::user::{UserEntity};
use crate::dtos::user_dto::UpdateUserDto;
use crate::errors::MyError;

pub struct UserRepository;

impl UserRepository {
  
  pub async fn create_user<E>(
    &self, 
    executor: &mut E,
    email: &str, 
    senha: &str
  ) -> Result<UserEntity, MyError>
    where
      for<'c> &'c mut E: sqlx::Executor<'c, Database = sqlx::Postgres>
  {
    
    let exists = self.user_exists(executor, email).await?;

    if exists == true {
      return Err(MyError::DatabaseError(sqlx::Error::PoolClosed));
    }

    let user = sqlx::query_as!(
      UserModel,    
      r#"INSERT INTO users (email, senha)
      VALUES ($1, $2)
      RETURNING id, email, senha
      "#,
      email,
      senha
    )
    .fetch_one(executor)
    .await
    .map_err(|_e| MyError::DatabaseError(sqlx::Error::PoolClosed))?;
    
    let user_domain = UserEntity::from(user); 
    
    Ok(user_domain)
  }

  pub async fn update_user<E>(
    &self,
    executor: &mut E,
    email: String,
    params: UpdateUserDto
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
        UPDATE users
        SET
          email = COALESCE($1, email),
          senha = COALESCE($2, senha)
          
        WHERE email = $3
        RETURNING id, email, senha
        "#,
        params.novo_email,
        params.nova_senha,
        email        
      )
      .fetch_one(executor)
      .await
      .map_err(|e| {
        println!("{e}");
        MyError::DatabaseError(sqlx::Error::PoolClosed)
      } )?;

      let domain_user = UserEntity::from(user);

      Ok(domain_user)
    }

  pub async fn find_user_by_email<E>(
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
    .await
    .map_err(|_e| MyError::DatabaseError(sqlx::Error::PoolClosed) )?;
    
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
        RETURNING id, email, senha      
      "#,
      email
    )
    .fetch_one(executor)
    .await
    .map_err(|_|MyError::DatabaseError(sqlx::Error::PoolClosed))?;

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
    .await
    .map_err(|e| {
      println!("{e}");
      MyError::DatabaseError(sqlx::Error::PoolClosed)
    })?;
    
    let exists = exists_user.unwrap();

    Ok(exists)
  }
}

