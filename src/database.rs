/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

use crate::config::Config;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    error::Api,
    opt::auth::Root,
    sql::Uuid,
    Error, Result, Surreal,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct StoredIfcModel {
    pub id: Option<String>,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
    pub file_content: Option<String>,
}

pub struct Database {
    pub client: Surreal<Client>,
    pub session_token: Uuid,
}

impl Database {
    /// Creates a new `Database` instance.
    ///
    /// # Arguments
    /// * `config` - The application configuration.
    ///
    /// # Returns
    /// A new `Database` instance.
    pub async fn new(config: &Config) -> Self {
        let client = Surreal::new::<Ws>(&config.database_url)
            .await
            .unwrap_or_else(|_| {
                panic!("Failed to connect to SurrealDB at {}", config.database_url)
            });

        client
            .signin(Root {
                username: &config.database_username,
                password: &config.database_password,
            })
            .await
            .expect("Failed to sign in to SurrealDB");

        Self {
            client,
            session_token: Uuid::new(),
        }
    }

    /// Saves an IFC model to the database.
    ///
    /// # Arguments
    /// * `model` - The IFC model upload data.
    ///
    /// # Returns
    /// A `Result` containing the saved model with its ID.
    pub async fn save_ifc_model(&self, model: StoredIfcModel) -> Result<StoredIfcModel> {
        let now = Utc::now();
        let stored_model = StoredIfcModel {
            id: None,
            name: model.name,
            version: model.version,
            description: model.description,
            created_at: now,
            updated_at: now,
            metadata: model.metadata,
            file_content: model.file_content,
        };
        self.client
            .create("ifc_models")
            .content(stored_model)
            .await?
            .take()
            .ok_or_else(|| {
                Error::Api(Api::ParseError(String::from(
                    "Failed to retrieve created IFC model",
                )))
            })
    }

    /// Retrieves an IFC model from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the IFC model to retrieve.
    ///
    /// # Returns
    /// A `Result` containing the retrieved IFC model, or an error if not found.
    pub async fn get_ifc_model(&self, id: String) -> Result<StoredIfcModel> {
        self.client
            .select(("ifc_models", id))
            .await?
            .take()
            .ok_or_else(|| {
                Error::Api(Api::ParseError(String::from(
                    "Failed to retrieve IFC model",
                )))
            })
    }
}
