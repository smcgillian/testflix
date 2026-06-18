use sqlx::PgPool;

use crate::{
    db::repositories::{
        customer_repository::CustomerRepository, media_repository::MediaRepository,
        service_repository::ServiceRepository, subscription_repository::SubscriptionRepository,
    },
    services::{
        customer_service::CustomerService, media_service::MediaService,
        service_service::ServiceService, subscription_service::SubscriptionService,
    },
};

#[derive(Clone)]
pub struct AppState {
    pub media_service: MediaService,
    pub service_service: ServiceService,
    pub customer_service: CustomerService,
    pub subscription_service: SubscriptionService,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        let media_repo = MediaRepository::new(pool.clone());
        let service_repo = ServiceRepository::new(pool.clone());
        let customer_repo = CustomerRepository::new(pool.clone());
        let subscription_repo = SubscriptionRepository::new(pool);

        Self {
            media_service: MediaService::new(media_repo),
            service_service: ServiceService::new(service_repo),
            customer_service: CustomerService::new(customer_repo),
            subscription_service: SubscriptionService::new(subscription_repo),
        }
    }
}
