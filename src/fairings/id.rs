/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

use rocket::{
    async_trait,
    fairing::{Fairing, Info, Kind},
    http::Header,
    Data, Request, Response,
};
use uuid::Uuid;

pub struct IdFairing;

#[async_trait]
impl Fairing for IdFairing {
    fn info(&self) -> Info {
        Info {
            name: "Request ID",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _: &mut Data<'_>) {
        req.local_cache(|| Uuid::new_v4().to_string());
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        res.set_header(Header::new(
            "X-Request-ID",
            req.local_cache::<String, _>(String::new).clone(),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::local::blocking::{Client, LocalResponse};
    use rocket::{build, routes};

    #[rocket::get("/test")]
    fn test_endpoint() -> &'static str {
        "Hello, world!"
    }

    struct TestContext {
        client: Client,
    }

    impl TestContext {
        fn new() -> Self {
            let rocket = build().attach(IdFairing).mount("/", routes![test_endpoint]);
            let client = Client::tracked(rocket).expect("valid rocket instance");
            TestContext { client }
        }

        fn get<'a>(&'a self, path: &'a str) -> LocalResponse<'a> {
            self.client.get(path).dispatch()
        }
    }

    #[test]
    fn test_request_id_header() {
        let ctx = TestContext::new();
        let response = ctx.get("/test");

        assert_eq!(response.status(), Status::Ok);
        assert!(response.headers().get_one("X-Request-ID").is_some());

        let request_id = response.headers().get_one("X-Request-ID").unwrap();
        assert!(!request_id.is_empty());
        assert_eq!(request_id.len(), 36);
    }

    #[test]
    fn test_multiple_requests_different_ids() {
        let ctx = TestContext::new();

        let response1 = ctx.get("/test");
        let response2 = ctx.get("/test");

        let id1 = response1.headers().get_one("X-Request-ID").unwrap();
        let id2 = response2.headers().get_one("X-Request-ID").unwrap();

        assert_ne!(id1, id2, "Different requests should have different IDs");
    }
}
