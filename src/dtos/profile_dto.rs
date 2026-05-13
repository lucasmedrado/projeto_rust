use serde::{Deserialize};
use validator::Validate;
use regex::Regex;
use once_cell::sync::Lazy;

use crate::domain::profile::ProfileEntity;

static RE_SLUG: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").unwrap()
});

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProfileDto {
  #[validate(length(min = 3, max = 100, message = "Nome completo deve ter entre 3 e 100 caracteres"))]
  pub full_name: Option<String>,

  #[validate(length(min = 3, max = 100, message = "Localização deve ter entre 3 e 100 caracteres"))]
  pub localization: Option<String>,

  #[validate(length(min = 15, max = 500, message = "O objetivo profissional deve ter entre 15 e 500 caracteres" ))]
  pub professional_objective: Option<String>,

  #[validate(
    length(min = 3, max = 50, message = "O slug deve ter entre 3 e 50 caracteres"),
    regex(path = "RE_SLUG", message = "O slug deve conter apenas letras minúsculas, números e hifens (ex: meu-projeto-123)")
  )]
  pub slug: String,

  #[validate(length(min = 15, max = 500, message = "A bio deve ter entre 20 e 500 caracteres" ))]
  pub bio: Option<String>,
  
}


#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProfileDto {
  #[validate(length(min = 3, max = 100, message = "Nome completo deve ter entre 3 e 100 caracteres"))]
  pub full_name: Option<String>,

  #[validate(length(min = 3, max = 100, message = "Localização deve ter entre 3 e 100 caracteres"))]
  pub localization: Option<String>,

  #[validate(length(min = 15, max = 500, message = "O objetivo profissional deve ter entre 15 e 500 caracteres" ))]
  pub professional_objective: Option<String>,

  #[validate(
    length(min = 3, max = 50, message = "O slug deve ter entre 3 e 50 caracteres"),
    regex(path = "RE_SLUG", message = "O slug deve conter apenas letras minúsculas, números e hifens (ex: meu-projeto-123)")
  )]
  pub slug: Option<String>,

  #[validate(length(min = 15, max = 500, message = "A bio deve ter entre 20 e 500 caracteres" ))]
  pub bio: Option<String>,
  
}


