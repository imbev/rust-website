use askama::Template;
use axum::{
    extract::{Form, State},
    response::{Html, Redirect},
};
use serde::Deserialize;

use crate::AppState;

#[derive(Template)]
#[template(path = "new_post.html")]
pub struct NewPostPageTemplate {}

pub async fn new_post_page() -> Html<String> {
    NewPostPageTemplate {}.to_string().into()
}

#[derive(Deserialize)]
pub struct NewPostPayload {
    title: String,
    content: String,
}

pub async fn new_post_endpoint(
    State(state): State<AppState>,
    Form(payload): Form<NewPostPayload>,
) -> Redirect {
    sqlx::query("INSERT INTO posts (title, content) VALUES ($1, $2);")
        .bind(payload.title)
        .bind(payload.content)
        .execute(&state.db)
        .await
        .unwrap();
    Redirect::to("/")
}
