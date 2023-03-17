use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Todo {
    pub id: i32,
    pub username: String,
    pub description: String,
    pub time: String,
    pub date: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct CreateEntryData {
    pub username: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct UpdateEntryData {
    pub username: String,
    pub description: String,
}

