use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct DataResponse<T>
where
    T: Serialize,
{
    pub data: T,
}

pub fn ok<T>(data: T) -> Json<DataResponse<T>>
where
    T: Serialize,
{
    Json(DataResponse { data })
}
