use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use log::info;
use model::user::{UserCreationResult, UserLoginToken, UserPayload};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;
use token::token_manager::TokenManager;

pub mod model;
pub mod token;

struct ServerData {
    db_pool: PgPool,
    token_manager: TokenManager,
}

async fn register(data: web::Data<ServerData>, payload: web::Json<UserPayload>) -> impl Responder {
    let result = sqlx::query_file!(
        "queries/register_user.sql",
        payload.username,
        payload.password
    )
    .fetch_one(&data.db_pool)
    .await;

    match result {
        Ok(record) => {
            let response = UserCreationResult { id: record.id };
            HttpResponse::Created().json(response)
        }
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to create user: {}", err))
        }
    }
}

async fn login(data: web::Data<ServerData>, payload: web::Json<UserPayload>) -> impl Responder {
    let result = sqlx::query_file!("queries/get_user.sql", payload.username, payload.password)
        .fetch_one(&data.db_pool)
        .await;

    match result {
        Ok(_) => {
            let response = UserLoginToken {
                token: rand::random(),
            };
            HttpResponse::Ok().json(response)
        }
        Err(err) => HttpResponse::InternalServerError()
            .body(format!("Could not receive the access token: {}", err)),
    }
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
    })
    .bind("127.0.0.1:".to_owned() + port)?
    .run()
    .await
}
