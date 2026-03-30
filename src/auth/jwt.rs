use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const SECRET: &[u8] = b"sua_chave_secreta_aqui";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,   
    pub email: String,
    pub exp: usize,    
    pub iat: usize,    
}

pub enum AuthError {
    TokenCreation,
    InvalidToken,
    ExpiredToken,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AuthError::TokenCreation  => write!(f, "Erro ao criar token"),
            AuthError::InvalidToken   => write!(f, "Token inválido"),
            AuthError::ExpiredToken   => write!(f, "Token expirado"),
        }
    }
}

pub fn generate_token(user_id: Uuid, email: &str) -> Result<String, AuthError> {
  let now = Utc::now();
  let exp = (now + Duration::hours(24)).timestamp() as usize;
  let iat = now.timestamp() as usize;

  let claims = Claims {
    sub: user_id.to_string(),
    email: email.to_string(),
    exp,
    iat
  };

  encode(
    &Header::default(), 
    &claims, 
    &EncodingKey::from_secret(SECRET),
  )
  .map_err(|_| AuthError::TokenCreation)
}

pub fn validate_token(token: &str) -> Result<Claims, AuthError> {
  let mut validation = Validation::default();
  validation.validate_exp = true;

  decode::<Claims>(
    token,
    &DecodingKey::from_secret(SECRET),
    &validation
  )
  .map(|data| data.claims)
  .map_err(|err| {
        use jsonwebtoken::errors::ErrorKind;
        match err.kind() {
            ErrorKind::ExpiredSignature => AuthError::ExpiredToken,
            _ => AuthError::InvalidToken,
        }
    })
}