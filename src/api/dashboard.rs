/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

use rocket::{
    get,
    http::Status,
    request::{self, FromRequest, Outcome},
    response::content::RawHtml as Html,
    Request,
};

use super::auth::COOKIE_TOKEN;

pub struct AuthenticatedUser(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<AuthenticatedUser, ()> {
        let cookies = request.cookies();
        if let Some(cookie) = cookies.get_private(COOKIE_TOKEN) {
            Outcome::Success(AuthenticatedUser(cookie.value().to_string()))
        } else {
            Outcome::Error((Status::Unauthorized, ()))
        }
    }
}

#[get("/dashboard")]
pub fn dashboard(user: AuthenticatedUser) -> Html<String> {
    Html(format!(
        "<h1>Welcome to your Dashboard!</h1><p>Your token is: {}</p>",
        user.0
    ))
}
