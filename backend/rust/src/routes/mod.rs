pub mod customers;
pub mod media;
pub mod services;
pub mod subscriptions;

use axum::{Router, routing::get};

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/media", get(media::list_media))
        .route("/media/:id", get(media::get_media))
        .route("/services", get(services::list_services))
        .route("/customers/:email", get(customers::get_customer))
        .route(
            "/customers/:email/subscriptions",
            get(customers::list_subscriptions),
        )
        .route("/subscriptions/:id", axum::routing::put(subscriptions::update_subscription))
}
