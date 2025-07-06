use sqlx::types::Uuid;

#[derive(sqlx::FromRow)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub content: String,
}
