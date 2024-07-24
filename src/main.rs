use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use model::user::{UserCreationResult, UserPayload};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

pub mod model;

struct AppState {
    pool: PgPool,
}

async fn register(data: web::Data<AppState>, payload: web::Json<UserPayload>) -> impl Responder {
    let result = sqlx::query_file!(
        "queries/register_user.sql",
        payload.username,
        payload.password
    )
    .fetch_one(&data.pool)
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect("No .env file found");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(
            &env::var("DATABASE_URL")
                .expect("DATABASE_URL must be specified within the .env file."),
        )
        .await
        .unwrap();

    let state = web::Data::new(AppState { pool });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/register", web::post().to(register))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
