use serde::{Serialize};

use crate::models::profile::ProfileModel;
use crate::dtos::profile_dto::UpdateProfileDto;
use crate::repository::profile_repository::ProfileSummaryModel;

#[derive(Serialize, Debug)]
pub struct ProfileEntity {  
  pub full_name: String,
  pub localization: String,
  pub professional_objective: Option<String>,
  pub slug: String,
  pub bio: Option<String>,    
}

impl From<ProfileModel> for ProfileEntity {
    fn from(profile: ProfileModel) -> Self {        

        ProfileEntity {            
            full_name: profile.full_name,
            localization: profile.localization,
            professional_objective: profile.professional_objective,
            slug: profile.slug,
            bio: profile.bio,            
        }
    }
}

impl From<UpdateProfileDto> for ProfileEntity {
  fn from(value: UpdateProfileDto) -> Self {
      ProfileEntity {
        full_name: value.full_name.unwrap(),
        localization: value.localization.unwrap(),
        professional_objective: value.professional_objective,
        slug: value.slug.unwrap(),
        bio: value.bio
      }
  }
}

impl From<ProfileSummaryModel> for ProfileEntity {
  fn from(value: ProfileSummaryModel) -> Self {
      ProfileEntity {
        full_name: value.full_name.unwrap(),
        localization: value.localization.unwrap(),
        professional_objective: value.professional_objective,
        slug: value.slug.unwrap(),
        bio: value.bio
      }
  }
}

