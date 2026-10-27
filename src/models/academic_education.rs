use chrono::NaiveDate;
use uuid::Uuid;

pub struct AcademicEducation {
  pub id: Uuid,
  pub profile_id: Uuid,
  pub institution: String,
  pub degree: Option<String>,
  pub field_of_study: Option<String>,
  pub start_date: NaiveDate,
  pub end_date: Option<NaiveDate>,  
}