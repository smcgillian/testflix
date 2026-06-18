use sqlx::PgPool;

use crate::{error::AppError, models::subscription::UpdatedSubscription};

#[derive(Clone)]
pub struct SubscriptionRepository {
    pool: PgPool,
}

impl SubscriptionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn service_exists(&self, service_id: i32) -> Result<bool, AppError> {
        let exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM services WHERE id = $1)")
            .bind(service_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(exists)
    }

    pub async fn update_subscription(
        &self,
        subscription_id: i32,
        service_id: i32,
        billing_period: i16,
    ) -> Result<Option<UpdatedSubscription>, AppError> {
        let sub = sqlx::query_as::<_, UpdatedSubscription>(
            r#"
            UPDATE subscriptions
            SET service_id = $1, billing_period = $2, started_at = now()
            WHERE id = $3
            RETURNING id, customer_id, service_id, billing_period, started_at, expires_at
            "#,
        )
        .bind(service_id)
        .bind(billing_period)
        .bind(subscription_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(sub)
    }
}
