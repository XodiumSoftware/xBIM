/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

use crate::config::Config;
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

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let config = req
            .rocket()
            .state::<Config>()
            .expect("Config not found in Rocket state");
        match req.headers().get_one("X-API-Key") {
            Some(key) if key == config.api_key => Outcome::Success(AuthGuard),
            _ => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Header;
    use rocket::local::blocking::{Client, LocalResponse};
    use rocket::{build, get, routes};

    #[get("/protected")]
    fn protected_endpoint(_ag: AuthGuard) -> &'static str {
        "Protected content"
    }

    struct TestContext {
        client: Client,
        api_key: String,
    }

    impl TestContext {
        fn new() -> Self {
            let config = Config::default();
            let api_key = config.api_key.clone();
            let rocket = build()
                .manage(config)
                .mount("/", routes![protected_endpoint]);
            let client = Client::tracked(rocket).expect("valid rocket instance");
            TestContext { client, api_key }
        }

        fn request_with_key(&self, key: &str) -> LocalResponse<'_> {
            self.client
                .get("/protected")
                .header(Header::new("X-API-Key", key.to_string()))
                .dispatch()
        }
    }

    #[test]
    fn test_api_key_authentication() {
        let ctx = TestContext::new();

        // Test valid key
        let valid_response = ctx.request_with_key(&ctx.api_key);
        assert_eq!(valid_response.status(), Status::Ok);
        assert_eq!(valid_response.into_string().unwrap(), "Protected content");

        // Test invalid key
        let invalid_response = ctx.request_with_key("invalid-key");
        assert_eq!(invalid_response.status(), Status::Unauthorized);
    }
}
