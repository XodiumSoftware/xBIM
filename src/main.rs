/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

pub mod api {
    pub mod database;
    pub mod github;
}

use crate::api::github::{github_callback, github_login, GitHub};
use rocket::{build, launch, routes, Build, Rocket};
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_oauth2::OAuth2;

/// Launches the Rocket application.
///
/// # Returns
/// A Rocket instance.
#[launch]
async fn rocket() -> Rocket<Build> {
    build()
        .mount("/", routes![github_login, github_callback])
        .attach(
            CorsOptions::default()
                .allowed_origins(AllowedOrigins::all())
                .to_cors()
                .expect("Failed to build CORS"),
        )
        .attach(OAuth2::<GitHub>::fairing("github"))
}
