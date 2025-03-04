/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Uuid;
use surrealdb::Surreal;

/// Represents the database operations.
pub struct Database {
    pub client: Surreal<Client>,
    pub session_token: Uuid,
}

impl Database {
    /// Creates a new `Database` instance.
    ///
    /// # Arguments
    ///
    /// * `client` - A `Surreal<Client>` instance for database operations.
    /// * `session_token` - A `Uuid` instance for the session token.
    ///
    /// # Returns
    ///
    /// A new `Database` instance.
    pub fn new(client: Surreal<Client>) -> Self {
        Self {
            client,
            session_token: Uuid::new(),
        }
    }

    /// Runs a query on the database.
    ///
    /// # Arguments
    /// * `query` - A string slice that holds the query to be executed.
    ///
    /// # Returns
    /// A `Result` which is `Ok` if the query was successful, or an error if it failed.
    pub async fn run_query(&self, query: &str) -> surrealdb::Result<()> {
        self.client.query(query).await.map(|_| ())
    }
}
