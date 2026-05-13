use sqlx::PgPool;
use crate::dtos::user_login_dto::{LoginRequest, LoginResponse};
use crate::domain::user_login::{UserLoginEntity};
use crate::repository::user_login_repository::UserLoginRepository;
use crate::errors::MyError;

pub struct UserLoginService;

impl UserLoginService {
    pub async fn login_user(pool: PgPool, email: String) -> Result<UserLoginEntity, MyError> {
      let mut tx = pool
      .begin()
      .await?;

      let user = UserLoginRepository;
      let user_login = user.find_user_by_email(&mut *tx, &email).await?; 
      
      tx.commit().await?;

      Ok(user_login)
    }
  }

