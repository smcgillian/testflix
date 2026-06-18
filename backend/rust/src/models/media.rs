use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct MediaItem {
    pub id: i32,
    pub title: String,
    pub media_type: String,
    pub description: String,
    pub duration_seconds: i32,
    pub genre: String,
    pub classification: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct MediaDetailBase {
    pub id: i32,
    pub title: String,
    pub media_type: String,
    pub description: String,
    pub duration_seconds: i32,
    pub genre: String,
    pub classification: String,
}

#[derive(Debug, Serialize)]
pub struct MediaDetail {
    pub id: i32,
    pub title: String,
    pub media_type: String,
    pub description: String,
    pub duration_seconds: i32,
    pub genre: String,
    pub classification: String,
    pub services: Vec<String>,
}
