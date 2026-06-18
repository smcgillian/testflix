use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct Service {
    pub id: i32,
    pub name: String,
    pub r#type: String,
}
