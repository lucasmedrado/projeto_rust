use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use crate::dtos::user_dto::{CreateUserDto, UpdateUserDto};
use crate::domain::user::{Email, UserEntity};
use crate::repository::user_repository::UserRepository;
use crate::errors::MyError;

pub struct UserService;

impl UserService {
    pub async fn create_user(pool: PgPool, dto: CreateUserDto) -> Result<UserEntity, MyError> {
      let mut tx = pool
      .begin()
      .await
      .map_err(|_e| MyError::DatabaseError(sqlx::Error::PoolClosed))?;

      let user = UserRepository;
      let email = Email::new(dto.email);
      let validated = Email::parse(email.as_str().to_string());
      let validated_email = validated.unwrap();
      let hash_password = UserEntity::hash_password(&dto.password_hash).unwrap();
      
      //let is_equal = UserEntity::verify_password(&dto.senha, &hash_password);
      //println!("{is_equal}");          

      let user_domain = user.create_user(&mut *tx, validated_email.as_str(), &hash_password).await.map_err(|_e| MyError::DatabaseError(sqlx::Error::PoolClosed))?;
      
      tx.commit().await.map_err(|_e| MyError::DatabaseError(sqlx::Error::PoolClosed))?;

      Ok(user_domain)
    }


    pub async fn update_user(pool: PgPool, email: String, dto: UpdateUserDto) -> Result<UserEntity, MyError> {
      let mut tx = pool
      .begin()
      .await
      .map_err(|e| {
        println!("{e}");
        MyError::NotFound
      })?;

      let user = UserRepository;  
      
      let user_update = user.update_user(&mut *tx, email , dto).await.map_err(|_e| MyError::NotFound)?;

      tx.commit().await.map_err(|e|{
        println!("{e}");
        MyError::DatabaseError(sqlx::Error::PoolClosed)
      })?;

      Ok(user_update)

    }

    pub async fn find_user(pool: PgPool, email: &str) -> Result<String, MyError> {
      let mut tx = pool
      .begin()
      .await
      .map_err(|_e| MyError::NotFound)?;

      let user = UserRepository;
      let find_user = user.find_user_by_email(&mut *tx, &email).await.map_err(|_e| MyError::NotFound)?;

      tx.commit().await.map_err(|_e| MyError::NotFound)?;

      Ok(find_user)
    }

    pub async fn delete_user(pool: PgPool, email: &str) -> Result<UserEntity, MyError> {
      let mut tx = pool
      .begin()
      .await
      .map_err(|_|MyError::NotFound)?;

      let user = UserRepository;
      let deletd_user = user.delete_user(&mut *tx, email).await?;

      tx.commit().await.map_err(|_e| MyError::NotFound)?;      

      Ok(deletd_user)
    }
}
