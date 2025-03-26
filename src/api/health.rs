/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

use chrono::Utc;
use rocket::{get, serde::json::Json};
use std::collections::HashMap;

/// Health check endpoint to confirm the service is running.
///
/// # Returns
/// A JSON response with the status, version, and timestamp.
#[get("/health")]
pub fn health() -> Json<HashMap<String, String>> {
    Json(HashMap::from([
        ("status".to_string(), "ok".to_string()),
        ("version".to_string(), env!("CARGO_PKG_VERSION").to_string()),
        ("timestamp".to_string(), Utc::now().to_rfc3339()),
    ]))
}
