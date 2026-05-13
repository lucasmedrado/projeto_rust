use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct ProfileModel {
  pub id: Uuid,
  pub user_id: Uuid,
  pub full_name: Option<String>,
  pub localization: Option<String>,
  pub professional_objective: Option<String>,
  pub slug: Option<String>,
  pub bio: Option<String>,
  pub is_public: bool,  
}