use sqlx::PgPool;

use crate::{error::AppError, models::service::Service};

#[derive(Clone)]
pub struct ServiceRepository {
    pool: PgPool,
}

impl ServiceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn list_services(&self) -> Result<Vec<Service>, AppError> {
        let services = sqlx::query_as::<_, Service>("SELECT id, name, type FROM services ORDER BY id")
            .fetch_all(&self.pool)
            .await?;

        Ok(services)
    }
}
