// •	В post.rs определите структуры:
// o	Post — пост с полями: id, title, content, author_id, created_at, updated_at.
// o	Структуры для запросов: создание (title, content) и обновление (title, content).
// o	Реализуйте метод new для создания нового поста.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub content: String,
    pub author_id: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Create {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Update {
    pub title: String,
    pub content: String,
}

impl Post {
    pub fn new(id: u64, title: String, content: String, author_id: u64) -> Self {
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
