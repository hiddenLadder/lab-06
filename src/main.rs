use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::Deserialize;
use tokio::sync::RwLock;

type AppState = Arc<RwLock<Option<u64>>>;

#[tokio::main]
async fn main() {
    let state = Arc::new(RwLock::new(None));
    let app = Router::new()
        .route(
            "/price",
            get(get_price).patch(set_price).delete(delete_price),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Deserialize)]
struct PriceDTO {
    price: u64,
}

async fn get_price(State(state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(*state.read().await))
}

async fn set_price(
    State(state): State<AppState>,
    Json(input): Json<PriceDTO>,
) -> Result<impl IntoResponse, StatusCode> {
    *state.write().await = Some(input.price);
    Ok(StatusCode::OK)
}

async fn delete_price(State(state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    *state.write().await = None;
    Ok(StatusCode::OK)
}
