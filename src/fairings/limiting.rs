/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

use rocket::{
    async_trait,
    fairing::{Fairing, Info, Kind},
    http::Status,
    Data, Request,
};
use std::{
    collections::HashMap,
    net::IpAddr,
    sync::Mutex,
    time::{Duration, Instant},
};

pub struct RateLimiter {
    requests: Mutex<HashMap<IpAddr, Vec<Instant>>>,
    limit: usize,
    window: Duration,
}

impl RateLimiter {
    pub fn new(limit: usize, seconds: u64) -> Self {
        RateLimiter {
            requests: Mutex::new(HashMap::new()),
            limit,
            window: Duration::from_secs(seconds),
        }
    }

    fn is_rate_limited(&self, ip: IpAddr) -> bool {
        let mut requests = self.requests.lock().unwrap();
        let now = Instant::now();
        let window_start = now - self.window;

        let timestamps = requests.entry(ip).or_default();
        timestamps.retain(|&t| t >= window_start);

        let limited = timestamps.len() >= self.limit;
        if !limited {
            timestamps.push(now);
        }
        limited
    }
}

#[async_trait]
impl Fairing for RateLimiter {
    fn info(&self) -> Info {
        Info {
            name: "Rate Limiter",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _: &mut Data<'_>) {
        if let Some(client_ip) = req.client_ip() {
            if self.is_rate_limited(client_ip) {
                req.set_error(Status::TooManyRequests);
            }
        }
    }
}
