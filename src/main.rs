use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use model::{
    post::Post,
    user::{User, UserCreationResult, UserPayload},
};
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, PgPool};

use std::{env, sync::Mutex};

pub mod model;

struct AppState {
    pool: PgPool,
    posts: Mutex<Vec<Post>>,
}

#[derive(Deserialize)]
struct PostPayload {
    title: String,
    body: String,
}

async fn post(data: web::Data<AppState>, payload: web::Json<PostPayload>) -> impl Responder {
    let mut posts = data.posts.lock().unwrap();
    posts.push(Post::new(
        payload.title.replace("\"", "\\\"").clone(),
        payload.body.replace("\"", "\\\"").clone(),
    ));
    HttpResponse::Ok().body("Post created successfully.")
}

async fn get_posts(data: web::Data<AppState>) -> impl Responder {
    let posts = data.posts.lock().unwrap();
    HttpResponse::Ok().json(&*posts)
}

async fn create_user(data: web::Data<AppState>, payload: web::Json<UserPayload>) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO users (username, password) VALUES ($1, $2) RETURNING id",
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

    let state = web::Data::new(AppState {
        pool,
        posts: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/post", web::post().to(post))
            .route("/get-posts", web::get().to(get_posts))
            .route("/new-user", web::post().to(create_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
