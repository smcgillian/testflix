use sqlx::PgPool;

use crate::{
    error::AppError,
    models::media::{MediaDetail, MediaDetailBase, MediaItem},
};

#[derive(Clone)]
pub struct MediaRepository {
    pool: PgPool,
}

impl MediaRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn customer_exists_by_email(&self, email: &str) -> Result<bool, AppError> {
        let exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM customers WHERE email = $1)")
            .bind(email)
            .fetch_one(&self.pool)
            .await?;

        Ok(exists)
    }

    pub async fn list_media_for_customer_email(
        &self,
        email: &str,
    ) -> Result<Vec<MediaItem>, AppError> {
        let items = sqlx::query_as::<_, MediaItem>(
            r#"
            SELECT DISTINCT
                m.id,
                m.title,
                mt.name AS media_type,
                COALESCE(m.description, '') AS description,
                COALESCE(m.duration_seconds, 0) AS duration_seconds,
                COALESCE(m.genre, '') AS genre,
                COALESCE(c.name, '') AS classification
            FROM media m
            JOIN media_services ms ON ms.media_id = m.id
            JOIN subscriptions s ON s.service_id = ms.service_id
            JOIN customers cu ON cu.id = s.customer_id
            JOIN media_types mt ON mt.id = m.media_type_id
            LEFT JOIN classifications c ON c.id = m.classification_id
            WHERE cu.email = $1
            ORDER BY m.id
            "#,
        )
        .bind(email)
        .fetch_all(&self.pool)
        .await?;

        Ok(items)
    }

    pub async fn get_media_detail(&self, id: i32) -> Result<Option<MediaDetail>, AppError> {
        let base = sqlx::query_as::<_, MediaDetailBase>(
            r#"
            SELECT
                m.id,
                m.title,
                mt.name AS media_type,
                COALESCE(m.description, '') AS description,
                COALESCE(m.duration_seconds, 0) AS duration_seconds,
                COALESCE(m.genre, '') AS genre,
                COALESCE(c.name, '') AS classification
            FROM media m
            JOIN media_types mt ON mt.id = m.media_type_id
            LEFT JOIN classifications c ON c.id = m.classification_id
            WHERE m.id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        let Some(base) = base else {
            return Ok(None);
        };

        let services = sqlx::query_scalar::<_, String>(
            r#"
            SELECT s.name
            FROM services s
            JOIN media_services ms ON ms.service_id = s.id
            WHERE ms.media_id = $1
            ORDER BY s.id
            "#,
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await?;

        Ok(Some(MediaDetail {
            id: base.id,
            title: base.title,
            media_type: base.media_type,
            description: base.description,
            duration_seconds: base.duration_seconds,
            genre: base.genre,
            classification: base.classification,
            services,
        }))
    }
}
