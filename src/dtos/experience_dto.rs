use chrono::{Local, NaiveDate};
use serde::{Deserialize};
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Validate)]
#[validate(schema(function = "validate_period"))]
pub struct CreateExperienceDto {
  
  #[validate(length(min = 2, max = 150, message = "Esse campo deve ter entre 2 e 100 caracteres"))]
  pub company_name: String,

  #[validate(length(min = 2, max = 150, message = "Esse campo deve ter entre 2 e 150 caracteres"))]
  pub position: String,

  #[validate(custom(function = "validate_past_or_present"))]
  pub start_date: NaiveDate,
  pub end_date: Option<NaiveDate>,  
  pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
#[validate(schema(function = "validate_update_period"))]
pub struct UpdateExperienceDto {
  
  #[validate(length(min = 2, max = 150, message = "Esse campo deve ter entre 2 e 100 caracteres"))]
  pub company_name: Option<String>,

  #[validate(length(min = 2, max = 150, message = "Esse campo deve ter entre 2 e 150 caracteres"))]
  pub position: Option<String>,

  #[validate(custom(function = "validate_past_or_present"))]
  pub start_date: Option<NaiveDate>,
  pub end_date: Option<NaiveDate>,  
  pub description: Option<String>,
}

pub fn validate_past_or_present(start_date: &NaiveDate) -> Result<(), ValidationError> {
  let now = Local::now().date_naive();

  if *start_date > now {
    let mut error = ValidationError::new("data_futura");
    error.message = Some("A data de início não pode ser maior que a data atual.".into());
    return Err(error)
  }  

  Ok(())
}

pub fn validate_period(dto: &CreateExperienceDto) -> Result<(), ValidationError> {

  let Some(end_date) = dto.end_date else {
    return Ok(());
  };

  if end_date < dto.start_date {
    let mut error = ValidationError::new("invalid_period");
    error.message = Some("A data final não pode ser menor que a data inicial.".into());
    return Err(error);
  }
  
  Ok(())
}

pub fn validate_update_period(dto: &UpdateExperienceDto) -> Result<(), ValidationError> {

  let (Some(end_date), Some(start_date)) = (dto.end_date, dto.start_date) else {
    return Ok(());
  };

  if end_date < start_date {
    let mut error = ValidationError::new("invalid_period");
    error.message = Some("A data final não pode ser menor que a data inicial.".into());
    return Err(error);
  }
  
  Ok(())
}