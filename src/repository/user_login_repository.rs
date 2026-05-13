use crate::models::user::UserModel;
use crate::domain::user_login::{UserLoginEntity}; 
use crate::errors::MyError;

pub struct UserLoginRepository;

impl UserLoginRepository {
  pub async fn find_user_by_email<E>(
    &self,
    executor: &mut E,
    email: &str
  ) -> Result<UserLoginEntity, MyError>
    where 
       for<'c> &'c mut E: sqlx::Executor<'c, Database = sqlx::Postgres>
  {
    let user = sqlx::query_as!( 
      UserModel,     
      "SELECT id, email, password_hash FROM users WHERE email = $1",
      email
    )
    .fetch_one(executor)
    .await?;
    
    let user_login = UserLoginEntity::from(user);

    Ok(user_login)
  }
}