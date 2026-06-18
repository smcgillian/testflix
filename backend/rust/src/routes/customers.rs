use axum::{
    Json,
    extract::{Path, State},
};

use crate::{
    api::responses::{DataResponse, ok},
    error::AppError,
    models::{customer::Customer, subscription::CustomerSubscription},
    state::AppState,
};

pub async fn get_customer(
    State(state): State<AppState>,
    Path(email): Path<String>,
) -> Result<Json<DataResponse<Customer>>, AppError> {
    let customer = state.customer_service.get_by_email(&email).await?;
    Ok(ok(customer))
}

pub async fn list_subscriptions(
    State(state): State<AppState>,
    Path(email): Path<String>,
) -> Result<Json<DataResponse<Vec<CustomerSubscription>>>, AppError> {
    let subs = state.customer_service.list_subscriptions_by_email(&email).await?;
    Ok(ok(subs))
}
