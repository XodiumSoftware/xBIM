/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

use rocket::{
    async_trait,
    fairing::{Fairing, Info, Kind},
    http::Status,
    Data, Request, Response,
};
use std::{collections::HashSet, net::IpAddr};

#[derive(Debug, Clone)]
pub struct IpFilter {
    pub allowed_ips: HashSet<IpAddr>,
    pub denied_ips: HashSet<IpAddr>,
    pub allow_all_by_default: bool,
}

impl Default for IpFilter {
    fn default() -> Self {
        Self {
            allowed_ips: HashSet::new(),
            denied_ips: HashSet::new(),
            allow_all_by_default: true,
        }
    }
}

impl IpFilter {
    pub fn new(
        allowed_ips: HashSet<IpAddr>,
        denied_ips: HashSet<IpAddr>,
        allow_all_by_default: bool,
    ) -> Self {
        Self {
            allowed_ips,
            denied_ips,
            allow_all_by_default,
        }
    }

    pub fn is_ip_allowed(&self, ip: Option<IpAddr>) -> bool {
        if let Some(ip) = ip {
            if self.denied_ips.contains(&ip) {
                return false;
            }
            if !self.allowed_ips.is_empty() {
                return self.allowed_ips.contains(&ip);
            }
            self.allow_all_by_default
        } else {
            self.allow_all_by_default
        }
    }
}

#[async_trait]
impl Fairing for IpFilter {
    fn info(&self) -> Info {
        Info {
            name: "Request IP Filter",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _: &mut Data<'_>) {
        if !self.is_ip_allowed(req.client_ip()) {
            req.local_cache(|| true);
            let ip_str = req
                .client_ip()
                .map_or("Unknown".to_string(), |ip| ip.to_string());
            println!("Blocked request from unauthorized IP: {}", ip_str);
        }
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        if *req.local_cache(|| false) {
            res.set_status(Status::Forbidden);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::{build, get, local::asynchronous::Client, routes};
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    #[get("/")]
    fn index() -> &'static str {
        "Hello, world!"
    }

    struct TestContext {
        client: Client,
    }

    impl TestContext {
        async fn with_filter(filter: IpFilter) -> Self {
            let rocket = build().attach(filter).mount("/", routes![index]);
            let client = Client::tracked(rocket)
                .await
                .expect("valid rocket instance");
            TestContext { client }
        }

        async fn default() -> Self {
            Self::with_filter(IpFilter::default()).await
        }

        async fn test_ip(&self, ip: Option<IpAddr>) -> Status {
            let req = match ip {
                Some(ip) => {
                    let socket_addr = SocketAddr::new(ip, 8000);
                    self.client.get("/").remote(socket_addr)
                }
                None => self.client.get("/"),
            };

            let response = req.dispatch().await;
            response.status()
        }
    }

    #[rocket::async_test]
    async fn test_allow_all_by_default() {
        let ctx = TestContext::default().await;
        let test_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1));

        assert_eq!(ctx.test_ip(Some(test_ip)).await, Status::Ok);
    }

    #[rocket::async_test]
    async fn test_denied_ip() {
        let mut denied_ips = HashSet::new();
        denied_ips.insert(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)));

        let filter = IpFilter::new(HashSet::new(), denied_ips, true);
        let ctx = TestContext::with_filter(filter).await;

        assert_eq!(
            ctx.test_ip(Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))))
                .await,
            Status::Forbidden
        );

        assert_eq!(
            ctx.test_ip(Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2))))
                .await,
            Status::Ok
        );
    }

    #[rocket::async_test]
    async fn test_allowed_ip() {
        let mut allowed_ips = HashSet::new();
        allowed_ips.insert(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)));

        let filter = IpFilter::new(allowed_ips, HashSet::new(), false);
        let ctx = TestContext::with_filter(filter).await;

        assert_eq!(
            ctx.test_ip(Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))))
                .await,
            Status::Ok
        );

        assert_eq!(
            ctx.test_ip(Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2))))
                .await,
            Status::Forbidden
        );
    }

    #[rocket::async_test]
    async fn test_denied_takes_precedence() {
        let mut allowed_ips = HashSet::new();
        allowed_ips.insert(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)));

        let mut denied_ips = HashSet::new();
        denied_ips.insert(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)));

        let filter = IpFilter::new(allowed_ips, denied_ips, true);
        let ctx = TestContext::with_filter(filter).await;

        assert_eq!(
            ctx.test_ip(Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))))
                .await,
            Status::Forbidden
        );
    }

    #[test]
    fn test_is_ip_allowed() {
        let mut allowed_ips = HashSet::new();
        allowed_ips.insert(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)));

        let mut denied_ips = HashSet::new();
        denied_ips.insert(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)));

        let filter = IpFilter::new(allowed_ips, denied_ips, false);
        assert!(filter.is_ip_allowed(Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)))));
        assert!(!filter.is_ip_allowed(Some(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)))));
        assert!(!filter.is_ip_allowed(Some(IpAddr::V4(Ipv4Addr::new(172, 16, 0, 1)))));
        assert!(!filter.is_ip_allowed(None));
    }
}
