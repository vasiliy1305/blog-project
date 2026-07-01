use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Registration {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}
