use sqlx::PgPool;

use crate::{
    error::AppError,
    models::{customer::Customer, subscription::CustomerSubscription},
};

#[derive(Clone)]
pub struct CustomerRepository {
    pool: PgPool,
}

impl CustomerRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_by_email(&self, email: &str) -> Result<Option<Customer>, AppError> {
        let customer = sqlx::query_as::<_, Customer>(
            "SELECT id, first_name, last_name, email FROM customers WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(customer)
    }

    pub async fn list_subscriptions_by_customer_id(
        &self,
        customer_id: i32,
    ) -> Result<Vec<CustomerSubscription>, AppError> {
        let subscriptions = sqlx::query_as::<_, CustomerSubscription>(
            r#"
            SELECT
                sub.id,
                sub.customer_id,
                sub.service_id,
                s.name AS service_name,
                s.type AS service_type,
                sub.billing_period,
                sub.started_at,
                sub.expires_at
            FROM subscriptions sub
            JOIN services s ON s.id = sub.service_id
            WHERE sub.customer_id = $1
            ORDER BY sub.id
            "#,
        )
        .bind(customer_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(subscriptions)
    }
}
