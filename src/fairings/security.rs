/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

use rocket::{
    async_trait,
    fairing::{Fairing, Info, Kind},
    http::Header,
    Request, Response,
};

/// This middleware supplements Rocket's built-in Shield fairing with additional security headers.
/// Shield provides basic protection with:
/// - X-Frame-Options: `SAMEORIGIN`
/// - X-Content-Type-Options: `nosniff`
/// - Permissions-Policy: `interest-cohort=()`
#[derive(Clone, Debug)]
pub struct SecurityHeaders {
    pub content_security_policy: Option<String>,
    pub xss_protection: Option<String>,
    pub content_type_options: Option<String>,
    pub frame_options: Option<String>,
    pub referrer_policy: Option<String>,
    pub strict_transport_security: Option<String>,
    pub permissions_policy: Option<String>,
}

impl Default for SecurityHeaders {
    fn default() -> Self {
        Self {
            content_security_policy: Some(
                "default-src 'self'; script-src 'self'; object-src 'none';".to_string(),
            ),
            xss_protection: Some("1; mode=block".to_string()),
            content_type_options: None,
            frame_options: None,
            referrer_policy: Some("strict-origin-when-cross-origin".to_string()),
            strict_transport_security: Some("max-age=31536000; includeSubDomains".to_string()),
            permissions_policy: Some("camera=(), microphone=(), geolocation=()".to_string()),
        }
    }
}

#[async_trait]
impl Fairing for SecurityHeaders {
    fn info(&self) -> Info {
        Info {
            name: "Response Security Headers",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _: &'r Request<'_>, res: &mut Response<'r>) {
        for (name, option_value) in [
            ("Content-Security-Policy", &self.content_security_policy),
            ("X-XSS-Protection", &self.xss_protection),
            ("X-Content-Type-Options", &self.content_type_options),
            ("X-Frame-Options", &self.frame_options),
            ("Referrer-Policy", &self.referrer_policy),
            ("Strict-Transport-Security", &self.strict_transport_security),
            ("Permissions-Policy", &self.permissions_policy),
        ] {
            if let Some(value) = option_value {
                res.set_header(Header::new(name, value.clone()));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::local::asynchronous::{Client, LocalResponse};
    use rocket::{async_test, build, get, routes};

    #[get("/")]
    fn index() -> &'static str {
        "Hello, world!"
    }

    struct TestContext {
        client: Client,
    }

    impl TestContext {
        async fn new(middleware: SecurityHeaders) -> Self {
            let rocket = build()
                .attach(middleware.clone())
                .mount("/", routes![index]);
            let client = Client::tracked(rocket)
                .await
                .expect("valid rocket instance");
            TestContext { client }
        }

        async fn default() -> Self {
            Self::new(SecurityHeaders::default()).await
        }

        async fn custom() -> Self {
            let custom_middleware = SecurityHeaders {
                content_security_policy: Some("default-src 'self' https://example.com".to_string()),
                xss_protection: None,
                content_type_options: Some("nosniff".to_string()),
                frame_options: Some("SAMEORIGIN".to_string()),
                referrer_policy: None,
                strict_transport_security: Some("max-age=63072000".to_string()),
                permissions_policy: None,
            };
            Self::new(custom_middleware).await
        }

        async fn no_headers() -> Self {
            let no_headers = SecurityHeaders {
                content_security_policy: None,
                xss_protection: None,
                content_type_options: None,
                frame_options: None,
                referrer_policy: None,
                strict_transport_security: None,
                permissions_policy: None,
            };
            Self::new(no_headers).await
        }

        async fn get<'a>(&'a self, path: &'a str) -> LocalResponse<'a> {
            self.client.get(path).dispatch().await
        }
    }

    #[test]
    fn test_fairing_info() {
        let rshm = SecurityHeaders::default();
        let info = rshm.info();
        assert_eq!(info.name, "Response Security Headers");
        assert!(info.kind.is(Kind::Response));
    }

    #[async_test]
    async fn test_default_security_headers() {
        let ctx = TestContext::default().await;
        let response = ctx.get("/").await;

        assert_eq!(response.status(), Status::Ok);

        let headers = response.headers();
        assert_eq!(
            headers.get_one("Content-Security-Policy"),
            Some("default-src 'self'; script-src 'self'; object-src 'none';")
        );
        assert_eq!(headers.get_one("X-XSS-Protection"), Some("1; mode=block"));
        assert_eq!(
            headers.get_one("Referrer-Policy"),
            Some("strict-origin-when-cross-origin")
        );
        assert_eq!(
            headers.get_one("Strict-Transport-Security"),
            Some("max-age=31536000; includeSubDomains")
        );
        assert_eq!(
            headers.get_one("Permissions-Policy"),
            Some("camera=(), microphone=(), geolocation=()")
        );
        assert_eq!(headers.get_one("X-Content-Type-Options"), Some("nosniff"));
        assert_eq!(headers.get_one("X-Frame-Options"), Some("SAMEORIGIN"));
    }

    #[async_test]
    async fn test_custom_security_headers() {
        let ctx = TestContext::custom().await;
        let response = ctx.get("/").await;

        assert_eq!(response.status(), Status::Ok);

        let headers = response.headers();
        assert_eq!(
            headers.get_one("Content-Security-Policy"),
            Some("default-src 'self' https://example.com")
        );
        assert_eq!(
            headers.get_one("Strict-Transport-Security"),
            Some("max-age=63072000")
        );
        assert_eq!(headers.get_one("X-XSS-Protection"), None);
        assert_eq!(headers.get_one("Referrer-Policy"), None);
        assert_eq!(headers.get_one("X-Content-Type-Options"), Some("nosniff"));
        assert_eq!(headers.get_one("X-Frame-Options"), Some("SAMEORIGIN"));
        assert!(headers.get_one("Permissions-Policy").is_some());
    }

    #[async_test]
    async fn test_no_security_headers() {
        let ctx = TestContext::no_headers().await;
        let response = ctx.get("/").await;

        assert_eq!(response.status(), Status::Ok);

        let headers = response.headers();
        assert_eq!(headers.get_one("Content-Security-Policy"), None);
        assert_eq!(headers.get_one("X-XSS-Protection"), None);
        assert_eq!(headers.get_one("Referrer-Policy"), None);
        assert_eq!(headers.get_one("Strict-Transport-Security"), None);
        assert_eq!(headers.get_one("X-Content-Type-Options"), Some("nosniff"));
        assert_eq!(headers.get_one("X-Frame-Options"), Some("SAMEORIGIN"));
        assert!(headers.get_one("Permissions-Policy").is_some());
    }
}
