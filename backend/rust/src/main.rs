mod api;
mod app;
mod config;
mod db;
mod error;
mod middleware;
mod models;
mod routes;
mod services;
mod state;

use std::net::SocketAddr;

use config::Config;
use sqlx::postgres::PgPoolOptions;
use tracing::info;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    init_tracing();

    let config = Config::from_env();

    let db_pool = PgPoolOptions::new()
        .max_connections(config.db_max_connections)
        .connect(&config.database_url)
        .await
        .unwrap_or_else(|err| panic!("failed to connect to database: {err}"));

    let app = app::build_app(db_pool, &config);

    serve(app, config.port).await;
}

async fn serve(app: axum::Router, port: u16) {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!(%addr, "server listening");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|err| panic!("failed to bind listener: {err}"));

    axum::serve(listener, app)
        .await
        .unwrap_or_else(|err| panic!("server error: {err}"));
}

fn init_tracing() {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "testflix_backend=info,tower_http=info,axum=info".into());

    tracing_subscriber::fmt().with_env_filter(filter).with_target(false).init();
}
