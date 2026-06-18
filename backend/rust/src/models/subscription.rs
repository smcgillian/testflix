use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct CustomerSubscription {
    pub id: i32,
    pub customer_id: i32,
    pub service_id: i32,
    pub service_name: String,
    pub service_type: String,
    pub billing_period: i16,
    pub started_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSubscriptionRequest {
    pub service_id: i32,
    pub billing_period: i16,
}

#[derive(Debug, Serialize, FromRow)]
pub struct UpdatedSubscription {
    pub id: i32,
    pub customer_id: i32,
    pub service_id: i32,
    pub billing_period: i16,
    pub started_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}
