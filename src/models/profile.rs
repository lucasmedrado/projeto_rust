use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct ProfileModel {
  pub id: Uuid,
  pub user_id: Uuid,
  pub full_name: String,
  pub localization: String,
  pub professional_objective: Option<String>,
  pub slug: String,
  pub bio: Option<String>,
  pub is_public: bool,  
}