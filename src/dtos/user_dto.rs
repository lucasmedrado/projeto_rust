use serde::{Serialize, Deserialize};
use validator::Validate;

use crate::domain::user::UserEntity;



#[derive(Deserialize, Validate)]
pub struct CreateUserDto {
  #[validate(email(message = "Email inválido"))]
  pub email: String,

  #[validate(length(min = 6, message = "A senha deve ter no mínimo 6 caracteres"))]
  //#[validate(regex(path = "RE_PASSWORD", message = "A senha deve conter letras e números"))]
  pub password_hash: String,
}

#[derive(Deserialize, Validate)]
pub struct EmailQueryDto {
  pub email: String,
}

#[derive(Deserialize, Validate)]
pub struct UpdateUserDto {
    pub email: Option<String>,
    pub password_hash: Option<String>
}

#[derive(Serialize)]
pub struct UserAuthedResponse {
    pub user: UserEntity,
    pub token: String,
}