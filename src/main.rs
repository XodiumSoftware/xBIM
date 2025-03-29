/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

pub mod fairings {
    pub mod compression;
    pub mod filtering;
    pub mod id;
    pub mod limiting;
    pub mod logging;
    pub mod security;
}

pub mod guards {
    pub mod auth;
    pub mod id;
}

pub mod routes {
    pub mod health;
    pub mod ifc;
}

pub mod config;
pub mod database;
pub mod errors;

use crate::fairings::limiting::RateLimiter;
use database::Database;
use errors::catchers;
use fairings::{
    compression::Compressor, filtering::IpFilter, id::IdFairing, logging::Logger,
    security::SecurityHeaders,
};
use rocket::{build, launch, routes, Build, Config, Rocket};
use rocket_cors::{AllowedOrigins, CorsOptions};
use routes::{
    health::health,
    ifc::{get_ifc_model, upload_ifc_model},
};
use std::process::exit;

#[launch]
async fn rocket() -> Rocket<Build> {
    let config = match config::Config::init() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            exit(1);
        }
    };
    build()
        .configure(Config {
            port: config.server_port,
            ..Config::debug_default()
        })
        .manage(config.clone())
        .manage(Database::new(&config).await)
        .mount("/", routes![health, upload_ifc_model, get_ifc_model])
        .attach(
            CorsOptions::default()
                .allowed_origins(AllowedOrigins::all())
                .to_cors()
                .expect("Failed to build CORS"),
        )
        .attach(Compressor)
        .attach(IdFairing)
        .attach(RateLimiter::new(100, 60))
        .attach(Logger)
        .attach(SecurityHeaders::default())
        .attach(IpFilter::default())
        .register("/", catchers())
}
