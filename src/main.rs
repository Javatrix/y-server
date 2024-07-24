use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use auth::{
    token::TokenResponse,
    token_manager::{self, TokenManager},
    user_auth_request::UserAuthRequest,
};
use dotenvy::dotenv;
use log::info;
use serde::ser::Impossible;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{env, sync::Mutex};
use user::user::{UserCreationResult, UserPayload};

mod auth;
mod user;

struct ServerData {
    db_pool: PgPool,
    token_manager: Mutex<TokenManager>,
}

async fn register(data: web::Data<ServerData>, payload: web::Json<UserPayload>) -> impl Responder {
    info!("Trying to register a user {}", payload.username);
    let result = sqlx::query_file!(
        "queries/register_user.sql",
        payload.username,
        payload.password
    )
    .fetch_one(&data.db_pool)
    .await;

    match result {
        Ok(record) => {
            info!("Registration for {} was successfull", payload.username);
            let response = UserCreationResult { id: record.id };
            HttpResponse::Created().json(response)
        }
        Err(err) => {
            info!("Registration for {} failed: {}", payload.username, err);
            HttpResponse::InternalServerError().body(format!("Failed to create user: {}", err))
        }
    }
}

async fn login(data: web::Data<ServerData>, payload: web::Json<UserPayload>) -> impl Responder {
    info!("Handling a login request for the user {}", payload.username);
    let result = sqlx::query_file!("queries/get_user.sql", payload.username, payload.password)
        .fetch_one(&data.db_pool)
        .await;

    match result {
        Ok(_) => {
            info!("Login for {} was successfull", payload.username);
            let mut token_manager = data.token_manager.lock().unwrap();
            let token = token_manager.create_token(&payload.username);
            let response = TokenResponse {
                token: token.get_value().clone(),
            };
            HttpResponse::Ok().json(response)
        }
        Err(err) => {
            info!("Login for {} failed", payload.username);
            HttpResponse::InternalServerError()
                .body(format!("Could not receive the access token: {}", err))
        }
    }
}

async fn access_something_secret(
    data: web::Data<ServerData>,
    payload: web::Json<UserAuthRequest>,
) -> impl Responder {
    let mut token_manager = data.token_manager.lock().unwrap();
    if token_manager.owns_valid_token(&payload.username)
        && token_manager
            .get_token(&payload.username)
            .unwrap()
            .get_value()
            .eq(&payload.token)
    {
        return HttpResponse::Ok().body("Secret content :)");
    }
    HttpResponse::Forbidden().body("You are not authorized to visit this endpoint.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect("No .env file found");
    env_logger::init();

    info!("Starting the server...");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(
            &env::var("DATABASE_URL")
                .expect("DATABASE_URL must be specified within the .env file."),
        )
        .await
        .unwrap();

    let state = web::Data::new(ServerData {
        db_pool: pool,
        token_manager: Default::default(),
    });

    let port =
        &env::var("Y_SERVER_PORT").expect("Y_SERVER_PORT must be specified within the .env file.");

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/register", web::post().to(register))
            .route("/login", web::get().to(login))
            .route("/secret", web::get().to(access_something_secret))
    })
    .bind("127.0.0.1:".to_owned() + port)?
    .run()
    .await
}
