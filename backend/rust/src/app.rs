use axum::{
    middleware,
    Router,
};
use sqlx::PgPool;
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

use crate::{config::Config, middleware::api_logger::log_api_traffic, routes, state::AppState};

pub fn build_app(pool: PgPool, config: &Config) -> Router {
    let state = AppState::new(pool);

    let api_router = routes::router()
        .route_layer(middleware::from_fn(log_api_traffic))
        .with_state(state);

    let static_files = ServeDir::new(&config.frontend_dir).not_found_service(ServeFile::new(
        format!("{}/index.html", config.frontend_dir),
    ));

    Router::new()
        .nest("/api", api_router)
        .fallback_service(static_files)
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}
