//! Axum boundary for facility handler.

use axum::{http::StatusCode, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct Response { pub capability: &'static str, pub status: &'static str, pub tenant_scoped: bool }

pub async fn handle() -> (StatusCode, Json<Response>) {
    (StatusCode::OK, Json(Response { capability: "facility-handler", status: "available", tenant_scoped: true }))
}
