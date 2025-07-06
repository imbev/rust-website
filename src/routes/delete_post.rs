use axum::{
    extract::{Path, State},
    response::Redirect,
};
use sqlx::types::Uuid;

use crate::AppState;

pub async fn delete_post_endpoint(
    State(state): State<AppState>,
    Path(post_id): Path<String>,
) -> Redirect {
    sqlx::query("DELETE FROM posts WHERE id = $1")
        .bind(Uuid::parse_str(&post_id).unwrap())
        .execute(&state.db)
        .await
        .unwrap();
    Redirect::to("/")
}
