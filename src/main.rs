/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

pub mod guards {
    pub mod auth;
    pub mod ratelimit;
}

pub mod models {
    pub mod user;
}

pub mod routes {
    pub mod github;
    pub mod health;
    pub mod ifc;
}

pub mod config;
pub mod database;
pub mod errors;
mod utils;

use crate::config::AppConfig;
use crate::routes::github::{github_callback, github_login, GitHubUser};
use crate::routes::health::health;
use crate::routes::ifc::{ifc_delete, ifc_get, ifc_update, ifc_upload};
use database::Database;
use errors::catchers;
use rocket::config::SecretKey;
use rocket::routes;
use rocket::{
    build, config::TlsConfig, launch, shield::ExpectCt, shield::Feature, shield::Frame,
    shield::Hsts, shield::NoSniff, shield::Permission, shield::Prefetch, shield::Referrer,
    shield::Shield, shield::XssFilter, time::Duration, Build, Config, Rocket,
};
use rocket_async_compression::{Compression, Level as CompressionLevel};
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_oauth2::{HyperRustlsAdapter, OAuth2, OAuthConfig, StaticProvider};

#[launch]
async fn rocket() -> Rocket<Build> {
    let config = AppConfig::new();
    build()
        .configure(Config {
            tls: (!config.tls_cert_path.is_empty() && !config.tls_key_path.is_empty())
                .then(|| TlsConfig::from_paths(&config.tls_cert_path, &config.tls_key_path)),
            secret_key: SecretKey::derive_from(config.secret_key.as_bytes()),
            ..Config::default()
        })
        .manage(config.clone())
        .manage(Database::new(&config).await)
        .mount(
            "/",
            routes![
                github_login,
                github_callback,
                health,
                ifc_upload,
                ifc_get,
                ifc_update,
                ifc_delete,
            ],
        )
        .attach(
            Shield::new()
                .enable(ExpectCt::Enforce(Duration::days(30)))
                .enable(
                    Permission::default()
                        .block(Feature::Camera)
                        .block(Feature::Geolocation)
                        .block(Feature::Microphone),
                )
                .enable(Frame::SameOrigin)
                .enable(Hsts::IncludeSubDomains(Duration::days(365)))
                .enable(NoSniff::Enable)
                .enable(Prefetch::On)
                .enable(Referrer::StrictOriginWhenCrossOrigin)
                .enable(XssFilter::EnableBlock),
        )
        .attach(
            CorsOptions::default()
                .allowed_origins(AllowedOrigins::all())
                .to_cors()
                .expect("Failed to build CORS"),
        )
        .attach(Compression::with_level(CompressionLevel::Default))
        .attach(OAuth2::<GitHubUser>::custom(
            HyperRustlsAdapter::default(),
            OAuthConfig::new(
                StaticProvider::GitHub,
                config.github_client_id.clone(),
                config.github_client_secret.clone(),
                Some(config.github_redirect_url.clone()),
            ),
        ))
        .register("/", catchers())
}
