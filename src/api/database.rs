/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

/// Configuration for the database connection.
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub db_name: String,
}

/// Represents the database operations.
pub struct Database {
    pub client: Surreal<Client>,
    pub config: DatabaseConfig,
    pub session_token: Option<String>,
}

impl Database {
    /// Creates a new `Database` instance.
    ///
    /// # Arguments
    ///
    /// * `client` - A `Surreal<Client>` instance for database operations.
    /// * `config` - A `DatabaseConfig` instance containing the database configuration.
    ///
    /// # Returns
    ///
    /// A new `Database` instance.
    pub fn new(client: Surreal<Client>, config: DatabaseConfig) -> Self {
        Self {
            client,
            config,
            session_token: None,
        }
    }

    /// Sets the session token for the database.
    ///
    /// # Arguments
    /// * `token` - A `String` representing the session token.
    pub fn set_session_token(&mut self, token: String) {
        self.session_token = Some(token);
    }

    /// Runs a query on the database.
    ///
    /// # Arguments
    /// * `query` - A string slice that holds the query to be executed.
    ///
    /// # Returns
    /// A `Result` which is `Ok` if the query was successful, or an error if it failed.
    pub async fn run_query(&self, query: &str) -> surrealdb::Result<()> {
        self.client.query(query).await?;
        Ok(())
    }
}
