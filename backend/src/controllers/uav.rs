use std::sync::Arc;

use axum::{
    extract::{Json, State},
    response::IntoResponse,
    routing::Router,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use utoipa::{IntoParams, ToSchema};

/// In-memory UAV position store
pub type UavStore = Mutex<Option<UavPosition>>;

/// UAV Position structure
#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct UavPosition {
    latitude: f64,
    longitude: f64,
}

impl Default for UavPosition {
    fn default() -> Self {
        UavPosition {
            latitude: 0.0,
            longitude: 0.0,
        }
    }
}

/// UAV operation errors
#[derive(Serialize, Deserialize, ToSchema)]
pub(super) enum UavError {
    #[schema(example = "UAV not found")]
    NotFound(String),
}


#[utoipa::path(
    get,
    path = "/api/uav",
    responses(
        (status = 200, description = "Current UAV position", body = [UavPosition])
    )
)]
pub async fn get_uav_position(State(store): State<Arc<UavStore>>) -> impl IntoResponse {
    let position = store.lock().await.clone().unwrap_or_default();
    (StatusCode::OK, Json(position)).into_response()
}


#[utoipa::path(
    post,
    path = "/api/uav/update",
    request_body = UavPosition,
    responses(
        (status = 200, description = "UAV position updated successfully", body = UavPosition),
    )
)]
pub async fn update_uav_position(
    State(store): State<Arc<UavStore>>,
    Json(new_position): Json<UavPosition>,
) -> impl IntoResponse {
    let mut store = store.lock().await;
    *store = Some(new_position.clone());
    (StatusCode::OK, Json(new_position)).into_response()
}
