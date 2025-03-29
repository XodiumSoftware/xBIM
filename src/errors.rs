/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

use rocket::{catch, catchers, http::Status, Catcher};

/// Returns a list of catchers for the application.
///
/// # Returns
/// A vector of catchers.
pub fn catchers() -> Vec<Catcher> {
    catchers![err_400, err_401, err_403, err_404, err_405, err_429, err_500, err_503]
}

#[catch(400)]
fn err_400() -> (Status, &'static str) {
    (
        Status::BadRequest,
        "Bad request format or invalid parameters",
    )
}

#[catch(401)]
fn err_401() -> (Status, &'static str) {
    (Status::Unauthorized, "Authentication required")
}

#[catch(403)]
fn err_403() -> (Status, &'static str) {
    (
        Status::Forbidden,
        "Access forbidden - You don't have permission to access this resource",
    )
}

#[catch(404)]
fn err_404() -> (Status, &'static str) {
    (Status::NotFound, "Resource not found")
}

#[catch(405)]
fn err_405() -> (Status, &'static str) {
    (
        Status::MethodNotAllowed,
        "Method not allowed for this resource",
    )
}

#[catch(429)]
fn err_429() -> (Status, &'static str) {
    (
        Status::TooManyRequests,
        "Rate limit exceeded - Please try again later",
    )
}

#[catch(500)]
fn err_500() -> (Status, &'static str) {
    (Status::InternalServerError, "Internal server error")
}

#[catch(503)]
fn err_503() -> (Status, &'static str) {
    (
        Status::ServiceUnavailable,
        "Service temporarily unavailable",
    )
}
