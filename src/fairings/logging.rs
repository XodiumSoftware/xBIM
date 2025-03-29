/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

use chrono::Utc;
use rocket::{
    async_trait,
    fairing::{Fairing, Info, Kind},
    Data, Request, Response,
};

pub struct Logger;

#[async_trait]
impl Fairing for Logger {
    fn info(&self) -> Info {
        Info {
            name: "Request and Response Logging",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _: &mut Data<'_>) {
        println!(
            "{} - Incoming request: {} {} from {}",
            Utc::now(),
            req.method(),
            req.uri(),
            req.client_ip()
                .map_or("Unknown".into(), |ip| ip.to_string())
        );
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        println!(
            "{} - Outgoing response: {} {} - Status: {}",
            Utc::now(),
            req.method(),
            req.uri(),
            res.status()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::{
        build, get,
        http::Status,
        local::blocking::{Client, LocalResponse},
        routes,
    };

    #[get("/test")]
    fn test_endpoint() -> &'static str {
        "Test successful"
    }

    struct TestContext {
        client: Client,
        logger: Logger,
    }

    impl TestContext {
        fn new() -> Self {
            let logger = Logger;
            let rocket = build().attach(Logger).mount("/", routes![test_endpoint]);
            let client = Client::tracked(rocket).expect("valid rocket instance");
            TestContext { client, logger }
        }

        fn request<'a>(&'a self, path: &'a str) -> LocalResponse<'a> {
            self.client.get(path).dispatch()
        }
    }

    #[test]
    fn test_logger_functionality() {
        let ctx = TestContext::new();

        // Test logger info
        let info = ctx.logger.info();
        assert_eq!(info.name, "Request and Response Logging");
        assert!(info.kind.is(Kind::Request));
        assert!(info.kind.is(Kind::Response));

        // Test fairing doesn't interfere with requests
        let response = ctx.request("/test");
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Test successful");
    }
}
