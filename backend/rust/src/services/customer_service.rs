use crate::{
    db::repositories::customer_repository::CustomerRepository,
    error::AppError,
    models::{customer::Customer, subscription::CustomerSubscription},
};

#[derive(Clone)]
pub struct CustomerService {
    repo: CustomerRepository,
}

impl CustomerService {
    pub fn new(repo: CustomerRepository) -> Self {
        Self { repo }
    }

    pub async fn get_by_email(&self, email: &str) -> Result<Customer, AppError> {
        let customer = self.repo.get_by_email(email).await?;

        customer.ok_or_else(|| AppError::NotFound("customer not found".to_string()))
    }

    pub async fn list_subscriptions_by_email(
        &self,
        email: &str,
    ) -> Result<Vec<CustomerSubscription>, AppError> {
        let customer = self.repo.get_by_email(email).await?;

        let customer = customer.ok_or_else(|| AppError::NotFound("customer not found".to_string()))?;

        self.repo.list_subscriptions_by_customer_id(customer.id).await
    }
}
