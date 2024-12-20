#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate rocket;

mod database;
mod schemas;

use crate::database::Database;
use rocket_cors::{AllowedOrigins, CorsOptions};
use tokio_postgres::NoTls;

#[launch]
async fn rocket() -> _ {
    let (client, conn) = tokio_postgres::connect(
        "host=localhost user=postgres password=postgres dbname=postgres",
        NoTls,
    )
    .await
    .expect("Failed to connect to Postgres");

    tokio::spawn(async move {
        if let Err(e) = conn.await {
            eprintln!("Failed to connect to Postgres: {}", e);
        }
    });

    rocket::build()
        .manage(
            Database::new(client)
                .await
                .expect("Failed to initialize the database"),
        )
        .mount("/", routes![])
        .attach(
            CorsOptions::default()
                .allowed_origins(AllowedOrigins::all())
                .to_cors()
                .expect("Error while building CORS"),
        )
}
