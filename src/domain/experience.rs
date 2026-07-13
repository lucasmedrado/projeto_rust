use serde::Serialize;
use chrono::NaiveDate;

use crate::models::experience::ExperienceModel;

#[derive(Debug, Serialize)]
pub struct ExperienceEntity {
  pub company_name: String,
  pub position: String,
  pub start_date: NaiveDate,
  pub end_date: Option<NaiveDate>,  
  pub description: Option<String>,
}

impl From<ExperienceModel> for ExperienceEntity {
  fn from(value: ExperienceModel) -> Self {
      Self {
        company_name: value.company_name,
        position: value.position,
        start_date: value.start_date,
        end_date: value.end_date,
        description: value.description
      }
  }
}

impl ExperienceEntity {
  pub fn date_range(start_date: NaiveDate, end_date: Option<NaiveDate>) -> Result<(NaiveDate, &'static str ), &'static str> {
    let Some(end) = end_date else {
      return Ok((start_date, "Atualmente"));
    };

    if start_date > end {
      return Err("A data de início (start_date) não pode ser maior que a data de término (end_date).");
    }

    Ok((start_date, "Atualmente"))
  } 
  
}