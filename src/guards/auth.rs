/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

use crate::routes::github::GitHubUser;
use rocket::{
    async_trait,
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

/// Authentication Guard
pub struct AuthGuard;

#[async_trait]
impl<'r> FromRequest<'r> for AuthGuard {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        request
            .cookies()
            .get_private("user_session")
            .and_then(|cookie| {
                serde_json::from_str::<GitHubUser>(cookie.value())
                    .map(|_| AuthGuard)
                    .ok()
            })
            .map(Outcome::Success)
            .unwrap_or(Outcome::Error((Status::Unauthorized, ())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Cookie;
    use rocket::local::asynchronous::Client;
    use rocket::{get, routes, tokio, Build, Rocket};
    use serde_json::json;

    #[get("/protected")]
    fn test_endpoint(_auth: AuthGuard) -> &'static str {
        "Authenticated!"
    }

    fn rocket_test() -> Rocket<Build> {
        rocket::build().mount("/", routes![test_endpoint])
    }

    #[tokio::test]
    async fn test_auth_guard_success() {
        let client = Client::tracked(rocket_test())
            .await
            .expect("valid rocket instance");

        let user_json = json!({
            "login": "testuser",
            "id": 12345,
            "name": "Test User"
        })
        .to_string();

        let cookie = Cookie::new("user_session", user_json);

        let response = client
            .get("/protected")
            .private_cookie(cookie)
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);
    }

    #[tokio::test]
    async fn test_auth_guard_unauthorized_no_cookie() {
        let client = Client::tracked(rocket_test())
            .await
            .expect("valid rocket instance");

        let response = client.get("/protected").dispatch().await;

        assert_eq!(response.status(), Status::Unauthorized);
    }

    #[tokio::test]
    async fn test_auth_guard_invalid_cookie_value() {
        let client = Client::tracked(rocket_test())
            .await
            .expect("valid rocket instance");

        let invalid_cookie = Cookie::new("user_session", "not_valid_json");

        let response = client
            .get("/protected")
            .private_cookie(invalid_cookie)
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Unauthorized);
    }
}
