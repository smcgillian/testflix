use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct Customer {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}
