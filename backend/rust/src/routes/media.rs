use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde::Deserialize;

use crate::{
    api::responses::ok,
    error::AppError,
    state::AppState,
};

#[derive(Deserialize)]
pub struct MediaQuery {
    pub email: Option<String>,
}

pub async fn list_media(
    State(state): State<AppState>,
    Query(query): Query<MediaQuery>,
) -> Result<Json<crate::api::responses::DataResponse<Vec<crate::models::media::MediaItem>>>, AppError>
{
    let email = query
        .email
        .ok_or_else(|| AppError::BadRequest("email query parameter is required".to_string()))?;

    let items = state.media_service.list_by_email(&email).await?;
    Ok(ok(items))
}

pub async fn get_media(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<crate::api::responses::DataResponse<crate::models::media::MediaDetail>>, AppError>
{
    let id = id
        .parse::<i32>()
        .map_err(|_| AppError::BadRequest("invalid media id".to_string()))?;

    let item = state.media_service.get_by_id(id).await?;
    Ok(ok(item))
}
