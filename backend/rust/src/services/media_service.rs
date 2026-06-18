use crate::{
    db::repositories::media_repository::MediaRepository,
    error::AppError,
    models::media::{MediaDetail, MediaItem},
};

#[derive(Clone)]
pub struct MediaService {
    repo: MediaRepository,
}

impl MediaService {
    pub fn new(repo: MediaRepository) -> Self {
        Self { repo }
    }

    pub async fn list_by_email(&self, email: &str) -> Result<Vec<MediaItem>, AppError> {
        if email.trim().is_empty() {
            return Err(AppError::BadRequest(
                "email query parameter is required".to_string(),
            ));
        }

        let exists = self.repo.customer_exists_by_email(email).await?;
        if !exists {
            return Err(AppError::NotFound("customer not found".to_string()));
        }

        self.repo.list_media_for_customer_email(email).await
    }

    pub async fn get_by_id(&self, id: i32) -> Result<MediaDetail, AppError> {
        let media = self.repo.get_media_detail(id).await?;

        media.ok_or_else(|| AppError::NotFound("media not found".to_string()))
    }
}
