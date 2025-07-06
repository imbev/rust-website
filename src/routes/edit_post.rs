use askama::Template;
use axum::{
    extract::{Form, Path, State},
    response::{Html, Redirect},
};
use serde::Deserialize;
use sqlx::types::Uuid;

use crate::{AppState, posts::Post};

#[derive(Template)]
#[template(path = "edit_post.html")]
pub struct EditPostPageTemplate {
    pub post: Post,
}

pub async fn edit_post_page(
    State(state): State<AppState>,
    Path(post_id): Path<String>,
) -> Html<String> {
    let post = sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE id = $1")
        .bind(Uuid::parse_str(&post_id).unwrap())
        .fetch_one(&state.db)
        .await
        .unwrap();
    EditPostPageTemplate { post }.to_string().into()
}

#[derive(Deserialize)]
pub struct EditPostPayload {
    id: String,
    title: String,
    content: String,
}

pub async fn edit_post_endpoint(
    State(state): State<AppState>,
    Form(payload): Form<EditPostPayload>,
) -> Redirect {
    sqlx::query("UPDATE posts SET title=$2, content=$3 WHERE id=$1")
        .bind(Uuid::parse_str(&payload.id).unwrap())
        .bind(payload.title)
        .bind(payload.content)
        .execute(&state.db)
        .await
        .unwrap();
    Redirect::to(&format!("/posts/{}/edit", payload.id))
}
