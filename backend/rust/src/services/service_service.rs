use crate::{db::repositories::service_repository::ServiceRepository, error::AppError, models::service::Service};

#[derive(Clone)]
pub struct ServiceService {
    repo: ServiceRepository,
}

impl ServiceService {
    pub fn new(repo: ServiceRepository) -> Self {
        Self { repo }
    }

    pub async fn list(&self) -> Result<Vec<Service>, AppError> {
        self.repo.list_services().await
    }
}
