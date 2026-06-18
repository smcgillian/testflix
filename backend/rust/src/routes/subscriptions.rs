use axum::{
    Json,
    body::Bytes,
    extract::{Path, State},
};

use crate::{
    api::responses::{DataResponse, ok},
    error::AppError,
    models::subscription::{UpdateSubscriptionRequest, UpdatedSubscription},
    state::AppState,
};

pub async fn update_subscription(
    State(state): State<AppState>,
    Path(id): Path<String>,
    body: Bytes,
) -> Result<Json<DataResponse<UpdatedSubscription>>, AppError> {
    let id = id
        .parse::<i32>()
        .map_err(|_| AppError::BadRequest("invalid subscription id".to_string()))?;

    let req: UpdateSubscriptionRequest = serde_json::from_slice(&body)
        .map_err(|_| AppError::BadRequest("invalid request body".to_string()))?;

    let updated = state.subscription_service.update(id, req).await?;
    Ok(ok(updated))
}
