/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */
use crate::guards::ratelimit::RateLimitGuard;
use crate::{database::Database, guards::auth::AuthGuard};
use chrono::{DateTime, Utc};
use rocket::{
    delete, get,
    http::Status,
    post, put,
    serde::json::Json,
    serde::{Deserialize, Serialize},
    State,
};
use rocket_governor::RocketGovernor;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct StoredIFC {
    pub id: Option<String>,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
    pub file_content: Option<String>,
}

/// Upload a new IFC model to the database.
///
/// # Arguments
/// * `database` - The database instance.
/// * `_authguard` - Authentication Guard.
/// * `_ratelimitguard` - Rate Limit Guard.
/// * `model` - The IFC model to upload.
///
/// # Returns
/// The saved IFC model with its ID.
#[post("/ifc", data = "<model>")]
pub async fn data_upload(
    database: &State<Database>,
    _authguard: AuthGuard,
    _ratelimitguard: RocketGovernor<'_, RateLimitGuard>,
    model: Json<StoredIFC>,
) -> Result<Json<StoredIFC>, Status> {
    println!("Processing IFC upload");
    match database.create("ifc_models", model.into_inner()).await {
        Ok(saved_model) => {
            println!("Successfully saved IFC model");
            Ok(Json(saved_model))
        }
        Err(e) => {
            println!("Error saving IFC model: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

/// Get an IFC model by ID.
///
/// # Arguments
/// * `database` - The database instance.
/// * `_authguard` - Authentication Guard.
/// * `_ratelimitguard` - Rate Limit Guard.
/// * `id` - The ID of the IFC model to retrieve.
///
/// # Returns
/// The retrieved IFC model.
#[get("/ifc/<id>")]
pub async fn data_get(
    database: &State<Database>,
    _authguard: AuthGuard,
    _ratelimitguard: RocketGovernor<'_, RateLimitGuard>,
    id: String,
) -> Result<Json<StoredIFC>, Status> {
    println!("Retrieving IFC model {}", id);
    match database.read::<StoredIFC>("ifc_models", &id).await {
        Ok(model) => {
            println!("Successfully retrieved IFC model {}", id);
            Ok(Json(model))
        }
        Err(e) => {
            println!("Error retrieving IFC model {}: {:?}", id, e);
            Err(Status::NotFound)
        }
    }
}

/// Update an existing IFC model.
///
/// # Arguments
/// * `database` - The database instance.
/// * `_authguard` - Authentication Guard.
/// * `_ratelimitguard` - Rate Limit Guard.
/// * `id` - The ID of the IFC model to update.
/// * `model` - The updated IFC model data.
///
/// # Returns
/// The updated IFC model.
#[put("/ifc/<id>", data = "<model>")]
pub async fn data_update(
    database: &State<Database>,
    _authguard: AuthGuard,
    _ratelimitguard: RocketGovernor<'_, RateLimitGuard>,
    id: String,
    model: Json<StoredIFC>,
) -> Result<Json<StoredIFC>, Status> {
    println!("Updating IFC model {}", id);
    match database.update("ifc_models", &id, model.into_inner()).await {
        Ok(updated_model) => {
            println!("Successfully updated IFC model {}", id);
            Ok(Json(updated_model))
        }
        Err(e) => {
            println!("Error updating IFC model {}: {:?}", id, e);
            Err(Status::InternalServerError)
        }
    }
}

/// Delete an IFC model by ID.
///
/// # Arguments
/// * `database` - The database instance.
/// * `_authguard` - Authentication Guard.
/// * `_ratelimitguard` - Rate Limit Guard.
/// * `id` - The ID of the IFC model to delete.
///
/// # Returns
/// 204 No Content on success, error status otherwise.
#[delete("/ifc/<id>")]
pub async fn data_delete(
    database: &State<Database>,
    _authguard: AuthGuard,
    _ratelimitguard: RocketGovernor<'_, RateLimitGuard>,
    id: String,
) -> Status {
    println!("Deleting IFC model {}", id);
    match database.delete::<StoredIFC>("ifc_models", &id).await {
        Ok(true) => {
            println!("Successfully deleted IFC model {}", id);
            Status::NoContent
        }
        Ok(false) => {
            println!("IFC model {} not found for deletion", id);
            Status::NotFound
        }
        Err(e) => {
            println!("Error deleting IFC model {}: {:?}", id, e);
            Status::InternalServerError
        }
    }
}
