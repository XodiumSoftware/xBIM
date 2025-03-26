/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

pub mod middlewares {
    pub mod auth;
}

pub mod routes {
    pub mod flutter_service_worker;
    pub mod health;
    pub mod ifc;
    pub mod index;
}

pub mod constants;
pub mod database;
pub mod errors;

use constants::ROCKET_PORT;
use database::Database;
use errors::catchers;
use rocket::{build, launch, routes, Build, Config, Rocket};
use rocket_cors::{AllowedOrigins, CorsOptions};
use routes::{
    flutter_service_worker::flutter_service_worker,
    health::health,
    ifc::{get_ifc_model, list_ifc_models, upload_ifc_model},
    index::index,
};

/// Launches the Rocket application.
///
/// # Returns
/// A Rocket instance.
#[launch]
async fn rocket() -> Rocket<Build> {
    build()
        .configure(Config {
            port: ROCKET_PORT,
            ..Config::debug_default()
        })
        .manage(Database::new().await)
        .mount(
            "/api",
            routes![
                health,
                flutter_service_worker,
                index,
                upload_ifc_model,
                get_ifc_model,
                list_ifc_models
            ],
        )
        .attach(
            CorsOptions::default()
                .allowed_origins(AllowedOrigins::all())
                .to_cors()
                .expect("Failed to build CORS"),
        )
        .register("/api", catchers())
}
