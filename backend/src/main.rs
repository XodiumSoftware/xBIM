#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

mod models {
    pub mod users;
}

mod api {
    pub mod users;
}

use api::users::create_user;
use dotenv::dotenv;
use rocket::{launch, routes};
use rocket_cors::{AllowedOrigins, CorsOptions};
use sea_orm::Database;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .manage(
            Database::connect("your_database_url_here")
                .await
                .expect("Failed to connect to database"),
        )
        .mount("/", routes![create_user])
        .attach(
            CorsOptions::default()
                .allowed_origins(AllowedOrigins::all())
                .to_cors()
                .expect("Failed to build CORS"),
        )
}
