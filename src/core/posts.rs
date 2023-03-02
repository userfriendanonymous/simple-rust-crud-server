use serde::Serialize;
use sqlx::{Pool, Postgres, FromRow};

#[derive(FromRow, Serialize)]
pub struct Post {
    title: String,
    content: String,
    description: String,
    id: i32,
}

pub async fn create(db: Pool<Postgres>, title: String, content: String, description: String) -> Result<i32, String> {
    match sqlx::query_as::<_, (i32,)>(
        "INSERT INTO posts (title, content, description) VALUES ($1, $2, $3) RETURNING id"
    )
    .bind(title.clone())
    .bind(content.clone())
    .bind(description.clone())
    .fetch_one(&db).await
    {
        Ok(result) => {
            Ok(result.0)
        },

        Err(error) => {
            Err(error.to_string())
        }
    }
}

pub async fn get_one(db: Pool<Postgres>, id: i32) -> Result<Post, String> {
    match sqlx::query_as::<_, Post>(
        "SELECT * FROM posts WHERE id = $1"
    )
    .bind(id)
    .fetch_one(&db).await
    {
        Ok(result) => {
            Ok(result)
        },

        Err(error) => {
            Err(error.to_string())
        }
    }
}

pub async fn get_all(db: Pool<Postgres>, limit: u32, skip: u32) -> Result<Vec<Post>, String> {
    match sqlx::query_as::<_, Post>(
        "SELECT * FROM posts LIMIT $1 OFFSET $2"
    )
    .bind(limit as i32)
    .bind(skip as i32)
    .fetch_all(&db).await
    {
        Ok(result) => {
            Ok(result)
        },

        Err(error) => {
            Err(error.to_string())
        }
    }
}