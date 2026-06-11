use axum::{Router, routing::get};
use tower_http::cors::CorsLayer;

use crate::pages;
use crate::db;

pub async fn app() -> Router {
    let state = db::create_pool().await;
    Router::new()
        //.route("/api/", get(pages::home))
        .with_state(state)
        .layer(CorsLayer::permissive())
}