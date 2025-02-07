/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

pub mod api {
    pub mod auth;
    pub mod dashboard;
    pub mod database;
}

use std::env;

use crate::api::auth::{callback, login, logout, GitHub};
use crate::api::dashboard::dashboard;
use crate::api::database::Database;
use rocket::{build, launch, routes, Build, Rocket};
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_oauth2::OAuth2;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::Surreal;

/// Launches the Rocket application.
///
/// # Returns
/// A Rocket instance.
#[launch]
async fn rocket() -> Rocket<Build> {
    dotenv::dotenv().ok();
    let _client_id = env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID not set");
    let _client_secret = env::var("GITHUB_CLIENT_SECRET").expect("GITHUB_CLIENT_SECRET not set");
    build()
        .manage(Database::new(
            Surreal::new::<Ws>("localhost:8000").await.unwrap(),
        ))
        .mount("/", routes![login, callback, logout, dashboard])
        .attach(
            CorsOptions::default()
                .allowed_origins(AllowedOrigins::all())
                .to_cors()
                .expect("Failed to build CORS"),
        )
        .attach(OAuth2::<GitHub>::fairing("github"))
}
