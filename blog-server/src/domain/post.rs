use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::FromRow)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreatePost {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdatePost {
    pub title: String,
    pub content: String,
}

impl Post {
    pub fn new(id: i64, title: String, content: String, author_id: i64) -> Self {
        Post {
            id,
            title,
            content,
            author_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
