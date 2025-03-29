/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

use crate::guards::{auth::AuthGuard, id::IdGuard};
use chrono::{DateTime, Utc};
use rocket::{get, serde::json::Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    status: String,
    version: String,
    timestamp: DateTime<Utc>,
    request_id: String,
}

/// Health check endpoint to confirm the service is running.
///
/// # Arguments
/// * `ig` - Identification Guard.
/// * `_ag` - Authentication Guard.
///
/// # Returns
/// A JSON response with the status, version, and timestamp.
#[get("/health")]
pub fn health(ig: IdGuard, _ag: AuthGuard) -> Json<Response> {
    println!("Health check requested with request ID: {}", ig.0);
    Json(Response {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: Utc::now(),
        request_id: ig.0,
    })
}
