use crate::{
    db::repositories::subscription_repository::SubscriptionRepository,
    error::AppError,
    models::subscription::{UpdateSubscriptionRequest, UpdatedSubscription},
};

#[derive(Clone)]
pub struct SubscriptionService {
    repo: SubscriptionRepository,
}

impl SubscriptionService {
    pub fn new(repo: SubscriptionRepository) -> Self {
        Self { repo }
    }

    pub async fn update(
        &self,
        subscription_id: i32,
        req: UpdateSubscriptionRequest,
    ) -> Result<UpdatedSubscription, AppError> {
        if req.billing_period != 1 && req.billing_period != 2 {
            return Err(AppError::BadRequest(
                "billing_period must be 1 or 2".to_string(),
            ));
        }

        let service_exists = self.repo.service_exists(req.service_id).await?;
        if !service_exists {
            return Err(AppError::NotFound("service not found".to_string()));
        }

        let updated = self
            .repo
            .update_subscription(subscription_id, req.service_id, req.billing_period)
            .await?;

        updated.ok_or_else(|| AppError::NotFound("subscription not found".to_string()))
    }
}
