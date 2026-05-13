use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    pub email: String,
    pub password_hash: String,
}

#[derive(Serialize, Validate)]
pub struct LoginResponse {
    pub token: String,    
}