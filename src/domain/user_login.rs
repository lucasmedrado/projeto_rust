use serde::Serialize;
use uuid::Uuid;
use crate::models::user::UserModel;

#[derive(Serialize, Debug)]
pub struct UserLoginEntity {
  pub id: Uuid,
  pub email: String,
  pub password_hash: String
}

impl From<UserModel> for UserLoginEntity {
    fn from(user: UserModel) -> Self {             

      UserLoginEntity {
        id: user.id,
        email: user.email,
        password_hash: user.password_hash
      }
    }
}