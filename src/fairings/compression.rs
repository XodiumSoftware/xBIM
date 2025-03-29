/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

use flate2::{write::GzEncoder, Compression};
use rocket::{
    async_trait,
    fairing::{Fairing, Info, Kind},
    http::Header,
    tokio::io::AsyncReadExt,
    Request, Response,
};
use std::io::{copy, Cursor};

pub struct Compressor;

#[async_trait]
impl Fairing for Compressor {
    fn info(&self) -> Info {
        Info {
            name: "Response Compressor",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _: &'r Request<'_>, res: &mut Response<'r>) {
        if !res.headers().contains("Content-Encoding") {
            let mut body = res.body_mut().take();
            let mut data = Vec::new();
            if body.read_to_end(&mut data).await.is_err() {
                return;
            }

            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            if copy(&mut data.as_slice(), &mut encoder).is_err() {
                return;
            }

            if let Ok(compressed_data) = encoder.finish() {
                res.set_header(Header::new("Content-Encoding", "gzip"));
                res.set_sized_body(compressed_data.len(), Cursor::new(compressed_data));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::read::GzDecoder;
    use rocket::{
        build, get,
        http::Status,
        local::blocking::{Client, LocalResponse},
        response::{Responder, Result},
        routes,
    };
    use std::io::Read;

    #[get("/large-response")]
    fn large_response() -> String {
        "A".repeat(1000)
    }

    struct PreCompressedResponse(Vec<u8>);

    impl<'r> Responder<'r, 'static> for PreCompressedResponse {
        fn respond_to(self, _: &'r Request<'_>) -> Result<'static> {
            let mut response = Response::new();
            response.set_header(Header::new("Content-Encoding", "br"));
            response.set_sized_body(self.0.len(), Cursor::new(self.0));
            Ok(response)
        }
    }

    #[get("/pre-compressed")]
    fn pre_compressed() -> PreCompressedResponse {
        PreCompressedResponse(b"Hello".to_vec())
    }

    struct TestContext {
        client: Client,
        compressor: Compressor,
    }

    impl TestContext {
        fn new() -> Self {
            let compressor = Compressor;
            let rocket = build()
                .attach(Compressor)
                .mount("/", routes![large_response, pre_compressed]);
            let client = Client::tracked(rocket).expect("valid rocket instance");
            TestContext { client, compressor }
        }

        fn get<'a>(&'a self, path: &'a str) -> LocalResponse<'a> {
            self.client.get(path).dispatch()
        }
    }

    #[test]
    fn test_compressor_info() {
        let ctx = TestContext::new();
        let info = ctx.compressor.info();

        assert_eq!(info.name, "Response Compression");
        assert!(info.kind.is(Kind::Response));
    }

    #[test]
    fn test_response_is_compressed() {
        let ctx = TestContext::new();
        let response = ctx.get("/large-response");

        assert_eq!(response.status(), Status::Ok);
        let headers = response.headers();

        assert!(headers.contains("Content-Encoding"));
        assert_eq!(headers.get_one("Content-Encoding").unwrap(), "gzip");

        let compressed_body = response.into_bytes().unwrap();
        assert!(compressed_body.len() < 1000);

        let mut decoder = GzDecoder::new(&compressed_body[..]);
        let mut decompressed = String::new();
        decoder
            .read_to_string(&mut decompressed)
            .expect("valid gzip data");

        assert_eq!(decompressed, "A".repeat(1000));
    }

    #[test]
    fn test_already_compressed_is_not_modified() {
        let ctx = TestContext::new();
        let response = ctx.get("/pre-compressed");

        assert_eq!(response.status(), Status::Ok);
        let headers = response.headers();

        assert!(headers.contains("Content-Encoding"));
        assert_eq!(headers.get_one("Content-Encoding").unwrap(), "br");
        assert_eq!(response.into_string().unwrap(), "Hello");
    }
}
