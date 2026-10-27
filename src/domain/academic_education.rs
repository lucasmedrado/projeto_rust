use serde::Serialize;
use chrono::NaiveDate;

pub struct AcademicEducationEntity {  
  pub institution: String,
  pub degree: Option<String>,
  pub field_of_study: Option<String>,  
}