use validator::validate_email;
use serde::Serialize;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

use crate::models::user::UserModel;


#[derive(Debug, Serialize)]
pub struct Email(String);

impl Email {

    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn parse(email: String) -> Result<Self, String> {
      if validate_email(&email) {
        Ok(Self(email))
      } else {
        Err("Formato de email inválido".to_string())
      }
    }

    pub fn as_str(&self) -> &str {
      &self.0
    }
}

#[derive(Serialize, Debug)]
pub struct UserEntity {    
    pub email: Email,
    pub password_hash: String
}



impl From<UserModel> for UserEntity {
    fn from(user: UserModel) -> Self {
        let validated_email = Email::parse(user.email).unwrap();

        UserEntity {
            email: validated_email,
            password_hash: user.password_hash
        }
    }
}


impl UserEntity {

    /*pub fn new(user: UserModel) -> Self {
        let validated_email = Email::parse(user.email).unwrap();
        //let validated_email = validated
        
        UserEntity {
            email: validated_email,
            senha: user.senha
        }
    }*/

    /// Transforma uma senha comum em um hash seguro para o banco de dados
    pub fn hash_password(password_hash: &String) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        
        // O hash resultante contém o algoritmo, parâmetros, salt e o hash propriamente dito
        let password_hash = argon2
            .hash_password(password_hash.as_bytes(), &salt)?
            .to_string();
            
        Ok(password_hash)
    }

    /// Verifica se a tentativa do usuário bate com o hash salvo
    pub fn verify_password(password_hash: &String, stored_hash: &str) -> bool {
        let Ok(parsed_hash) = PasswordHash::new(stored_hash) else {
            return false;
        };
        
        Argon2::default()
            .verify_password(password_hash.as_bytes(), &parsed_hash)
            .is_ok()
    }
}

