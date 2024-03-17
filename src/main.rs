use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

type AppState = Arc<RwLock<Option<u64>>>;

#[tokio::main]
async fn main() {
    let state = Arc::new(RwLock::new(None));
    let app = app(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

fn app(state: AppState) -> Router {
    Router::new()
        .route(
            "/price",
            get(get_price).patch(set_price).delete(delete_price),
        )
        .with_state(state)
}

#[derive(Debug, Serialize, Deserialize)]
struct PriceDTO {
    price: u64,
}

async fn get_price(State(state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    match *state.read().await {
        Some(price) => Ok(Json(price)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn set_price(
    State(state): State<AppState>,
    Json(input): Json<PriceDTO>,
) -> Result<impl IntoResponse, StatusCode> {
    *state.write().await = Some(input.price);
    Ok(Json(input.price))
}

async fn delete_price(State(state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    *state.write().await = None;
    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::axum_test::TestServer;

    #[tokio::test]
    async fn test_get_price() {
        let state: AppState = Arc::new(RwLock::new(Some(13)));
        let app = app(state);
        let server = TestServer::new(app).unwrap();

        let response = server.get("/price").await;

        assert_eq!(response.text(), "13");
    }

    #[tokio::test]
    async fn test_get_price_not_found() {
        let state: AppState = Arc::new(RwLock::new(None));
        let app = app(state);
        let server = TestServer::new(app).unwrap();

        let response = server.get("/price").await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_set_price() {
        let state: AppState = Arc::new(RwLock::new(None));
        let app = app(state);
        let server = TestServer::new(app).unwrap();

        let response1 = server.patch("/price").json(&PriceDTO { price: 13 }).await;
        let response2 = server.get("/price").await;

        assert_eq!(response1.text(), response2.text());
    }

    #[tokio::test]
    async fn test_delete_price() {
        let state: AppState = Arc::new(RwLock::new(Some(13)));
        let app = app(state);
        let server = TestServer::new(app).unwrap();

        let response = server.delete("/price").await;
        assert_eq!(response.status_code(), StatusCode::NO_CONTENT);
    }
}
