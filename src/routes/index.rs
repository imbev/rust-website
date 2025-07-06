use askama::Template;
use axum::{extract::State, response::Html};

use crate::{AppState, posts::Post};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexPageTemplate {
    posts: Vec<Post>,
}

pub async fn index_page(State(state): State<AppState>) -> Html<String> {
    let mut posts: Vec<Post> = sqlx::query_as::<_, Post>("SELECT * FROM posts")
        .fetch_all(&state.db)
        .await
        .unwrap();

    posts.reverse();

    IndexPageTemplate { posts }.to_string().into()
}
