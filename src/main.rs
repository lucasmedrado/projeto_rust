mod models;
mod domain;
mod dtos;
mod validated_json;
mod repository;
mod services;
mod controllers;
mod errors;
mod auth;
mod login;

use controllers::user_controller::{
    create_user_handler, 
    find_user_handler, 
    update_user_handler,
    delete_user_handler,
};

use controllers::profile_controller::{
    create_profile_handler,
    update_profile_handler,
    find_profile_handler
};

use controllers::experience_controller::{create_experience_handler};

use controllers::user_login_controller::{login_handler};

use axum::{routing::{post, get, patch, delete}, Router};
use sqlx::postgres::PgPoolOptions;
use std::env;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {    
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL deve estar configurada no arquivo .env");

    let pool = PgPoolOptions::new()
        .max_connections(5) // Limite de conexões simultâneas
        .connect(&database_url)
        .await
        .expect("Não foi possível conectar ao banco de dados");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    let app = Router::new()
        .route("/", post(login_handler))
        .route("/users", post(create_user_handler))
        .route("/profiles", post(create_profile_handler))
        .route("/profiles", patch(update_profile_handler))
        .route("/profiles", get(find_profile_handler))
        .route("/experiences", post(create_experience_handler))
        .route("/users", get(find_user_handler))
        .route("/users/:email", patch(update_user_handler))
        .route("/users/:email", delete(delete_user_handler))
        .with_state(pool);

    println!("🚀 Servidor rodando em http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();     
   
}
