use axum::{Json, extract::State};

use crate::{
    api::responses::{DataResponse, ok},
    error::AppError,
    models::service::Service,
    state::AppState,
};

pub async fn list_services(
    State(state): State<AppState>,
) -> Result<Json<DataResponse<Vec<Service>>>, AppError> {
    let services = state.service_service.list().await?;
    Ok(ok(services))
}
