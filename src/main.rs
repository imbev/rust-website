use std::env;

mod posts;
mod routes;

use axum::Router;
use axum::routing::{get, post};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, migrate};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

use crate::routes::delete_post::delete_post_endpoint;
use crate::routes::edit_post::{edit_post_endpoint, edit_post_page};
use crate::routes::index::index_page;
use crate::routes::new_post::{new_post_endpoint, new_post_page};

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    migrate!().run(&pool).await.unwrap();

    let state = AppState { db: pool };

    let app = Router::new()
        .route("/", get(index_page))
        .route("/new_post", get(new_post_page))
        .route("/posts", post(new_post_endpoint))
        .route("/posts/{post_id}/delete", get(delete_post_endpoint))
        .route("/posts/{post_id}/edit", get(edit_post_page))
        .route("/edit_post", post(edit_post_endpoint))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    let host = env::var("APP_HOST").unwrap();
    let port = env::var("APP_PORT").unwrap();

    let listener = TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();

    println!("Serving on http://{}:{}", host, port);

    axum::serve(listener, app).await.unwrap();
}
