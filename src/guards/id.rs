/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

use rocket::{
    async_trait,
    request::{FromRequest, Outcome},
    Request,
};

/// Identification Guard
pub struct IdGuard(pub String);

#[async_trait]
impl<'r> FromRequest<'r> for IdGuard {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Success(IdGuard(req.local_cache::<String, _>(String::new).clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fairings::id::IdFairing;
    use rocket::http::Status;
    use rocket::local::blocking::{Client, LocalResponse};
    use rocket::{build, routes};

    #[rocket::get("/test")]
    fn test_endpoint() -> &'static str {
        "Hello, world!"
    }

    #[rocket::get("/guard")]
    fn guard_endpoint(ig: IdGuard) -> String {
        format!("Request ID: {}", ig.0)
    }

    struct TestContext {
        client: Client,
    }

    impl TestContext {
        fn new() -> Self {
            let rocket = build()
                .attach(IdFairing)
                .mount("/", routes![test_endpoint, guard_endpoint]);
            let client = Client::tracked(rocket).expect("valid rocket instance");
            TestContext { client }
        }

        fn get<'a>(&'a self, path: &'a str) -> LocalResponse<'a> {
            self.client.get(path).dispatch()
        }
    }

    #[test]
    fn test_id_guard() {
        let ctx = TestContext::new();
        let response = ctx.get("/guard");

        assert_eq!(response.status(), Status::Ok);

        let body = response.into_string().unwrap();
        assert!(body.starts_with("Request ID: "));

        let request_id = body.strip_prefix("Request ID: ").unwrap();
        assert_eq!(request_id.len(), 36);
    }
}
