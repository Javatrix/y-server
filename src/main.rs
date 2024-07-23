use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use model::post::Post;
use serde::Deserialize;

use std::sync::Mutex;

pub mod model;

struct AppState {
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        posts: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/post", web::post().to(post))
            .route("/get-posts", web::get().to(get_posts))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
