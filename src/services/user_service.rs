use sqlx::PgPool;

use crate::dtos::user_dto::{CreateUserDto, UpdateUserDto};
use crate::domain::user::{self, Email, UserEntity};
use crate::repository::user_repository::{self, UserRepository};
use crate::errors::MyError;

pub struct UserService;

impl UserService {
    pub async fn create_user(pool: PgPool, dto: CreateUserDto) -> Result<UserEntity, MyError> {
      let mut tx = pool
      .begin()
      .await?;

      let user = UserRepository;
      let email = Email::new(dto.email); 
      let validated = Email::parse(email.as_str().to_string());
      let validated_email = validated.unwrap();
      let hash_password = UserEntity::hash_password(Some(&dto.password_hash)).unwrap();             

      let user_domain = user.create_user(&mut *tx, validated_email.as_str(), &hash_password.unwrap()).await.map_err(|_e| {
        
        MyError::DatabaseError(sqlx::Error::PoolClosed) 
      })?;
      
      tx.commit().await?;

      Ok(user_domain)
    }


    pub async fn update_user(pool: PgPool, email: String, mut dto: UpdateUserDto) -> Result<UserEntity, MyError> {
      let mut tx = pool
      .begin()
      .await
      .map_err(|_| MyError::NotFound)?;    

      let user_email:String = UserService::find_user(&pool, &email).await?;   

      match email != user_email.as_str() {
        true => return Err(MyError::NotFound),
        false => (),
      }    
      
      let password = UserEntity::hash_password(dto.password_hash.as_ref()).unwrap();      
      
      let user_repository = UserRepository;
      let user_domain = user_repository.find_user(&mut *tx, &email).await?;
      let stored_hash= user_domain.password_hash.as_str();      
      
      let verify_password = UserEntity::verify_password(dto.password_hash.as_ref().unwrap(), stored_hash);      
      
      match verify_password {
        false => return Err(MyError::NotFound),
        true => ()
      }

      let user = UserRepository;  
      dto.password_hash = password;
      let user_update = user.update_user(&mut *tx, email , dto).await.map_err(|_e| MyError::NotFound)?;

      tx.commit().await.map_err(|e| MyError::DatabaseError(sqlx::Error::PoolClosed))?;

      Ok(user_update)

    }

    pub async fn find_user(pool: &PgPool, email: &str) -> Result<String, MyError> {
      let mut tx = pool
      .begin()
      .await
      .map_err(|_e| MyError::NotFound)?;  

      let user = UserRepository;

      let find_user = user.find_user_by_email(&mut *tx, &email).await.map_err(|_e| MyError::NotFound)?;

       match email != find_user.as_str() {
        true => return Err(MyError::NotFound),
        false => (),
      }        

      tx.commit().await.map_err(|_e| MyError::NotFound)?;

      Ok(find_user)
    }

    pub async fn delete_user(pool: PgPool, email: &str) -> Result<UserEntity, MyError> {
      let mut tx = pool
      .begin()
      .await
      .map_err(|_| MyError::NotFound)?;

      let user_email:String = UserService::find_user(&pool, &email).await?;   

      match email != user_email.as_str() {
        true => return Err(MyError::NotFound),
        false => (),
      }   

      let user = UserRepository;
      let deletd_user = user.delete_user(&mut *tx, email).await?;

      tx.commit().await.map_err(|_e| MyError::NotFound)?;      

      Ok(deletd_user)
    }
}
