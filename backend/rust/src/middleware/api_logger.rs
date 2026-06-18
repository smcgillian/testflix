use axum::{
    body::{Body, to_bytes},
    extract::Request,
    middleware::Next,
    response::Response,
};
use tracing::info;

const MAX_LOG_BYTES: usize = 1024 * 1024;

pub async fn log_api_traffic(request: Request, next: Next) -> Response {
    let (parts, body) = request.into_parts();

    let method = parts.method.clone();
    let path = parts.uri.path().to_string();
    let query = parts.uri.query().map(|q| q.to_string());

    let request_bytes = match to_bytes(body, MAX_LOG_BYTES).await {
        Ok(bytes) => bytes,
        Err(_) => {
            info!(
                method = %method,
                path = %path,
                query = ?query,
                request_body = "<failed to read request body>",
                "api request"
            );
            return next.run(Request::from_parts(parts, Body::empty())).await;
        }
    };

    let request_body = String::from_utf8_lossy(&request_bytes);
    info!(
        method = %method,
        path = %path,
        query = ?query,
        request_body = %request_body,
        "api request"
    );

    let request = Request::from_parts(parts, Body::from(request_bytes));
    let response = next.run(request).await;

    let (response_parts, response_body) = response.into_parts();
    let status = response_parts.status;

    let response_bytes = match to_bytes(response_body, MAX_LOG_BYTES).await {
        Ok(bytes) => bytes,
        Err(_) => {
            info!(
                method = %method,
                path = %path,
                status = %status,
                response_body = "<failed to read response body>",
                "api response"
            );
            return Response::from_parts(response_parts, Body::empty());
        }
    };

    let response_body_str = String::from_utf8_lossy(&response_bytes);
    info!(
        method = %method,
        path = %path,
        status = %status,
        response_body = %response_body_str,
        "api response"
    );

    Response::from_parts(response_parts, Body::from(response_bytes))
}
