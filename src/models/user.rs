use uuid::Uuid;
#[derive(Debug, sqlx::FromRow)]
pub struct UserModel {
  pub id: Uuid,  
  pub email: String,
  pub senha: String
}