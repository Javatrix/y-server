use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use auth::{token::TokenResponse, token_manager::TokenManager};
use dotenvy::dotenv;
use log::info;
use post::post::PostCreationPayload;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{env, sync::Mutex};
use user::user::{UserIdResponse, UserPayload};

mod auth;
mod post;
mod user;

struct ServerData {
    db_pool: PgPool,
    token_manager: Mutex<TokenManager>,
}

async fn get_username(pool: &PgPool, id: i32) -> Option<String> {
    let result = sqlx::query_file!("queries/id_to_username.sql", id)
        .fetch_one(pool)
        .await;
    if result.is_ok() {
        return Some(result.unwrap().username);
    }
    None
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
            info!("Registration for {} was successful", payload.username);
            let response = UserIdResponse { id: record.id };
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
            info!("Login for {} was successful", payload.username);
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

async fn create_post(
    data: web::Data<ServerData>,
    payload: web::Json<PostCreationPayload>,
) -> impl Responder {
    let token_manager = data.token_manager.lock().unwrap();
    let username = get_username(&data.db_pool, payload.author_id).await;
    if username.is_none() {
        info!(
            "Tried to create post with invalid user id: {}",
            payload.author_id
        );
        return HttpResponse::BadRequest().body("Invalid user id");
    }
    let username = username.unwrap();
    if token_manager.is_valid(&username, &payload.token) {
        info!(
            "User {username} successfully authenticated with token {}",
            payload.token
        );
        let result = sqlx::query_file!(
            "queries/create_post.sql",
            payload.author_id,
            payload.title,
            payload.body
        )
        .execute(&data.db_pool)
        .await;

        if result.is_ok() {
            return HttpResponse::Ok().body("Post created successfully");
        } else {
            return HttpResponse::InternalServerError()
                .body(format!("Post creation failed: {}", result.unwrap_err()));
        }
    } else {
        return HttpResponse::Forbidden().body("Invalid token or user id");
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
            .route("/post", web::post().to(create_post))
    })
    .bind("127.0.0.1:".to_owned() + port)?
    .run()
    .await
}
