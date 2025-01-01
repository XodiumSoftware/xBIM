#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

use crate::models::users::{ActiveModel, Model};
use rocket::serde::json::Json;
use rocket::{post, State};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct CreateUserRequest {
    username: String,
    email: String,
}

#[post("/users", data = "<user_request>")]
pub(crate) async fn create_user(
    user_request: Json<CreateUserRequest>,
    db: &State<DatabaseConnection>,
) -> Json<Result<Model, String>> {
    let user_request = user_request.into_inner();
    let new_user = ActiveModel {
        username: Set(user_request.username),
        email: Set(user_request.email),
        ..Default::default()
    };

    match new_user.insert(db.inner()).await {
        Ok(user) => Json(Ok(user)),
        Err(err) => Json(Err(err.to_string())),
    }
}
