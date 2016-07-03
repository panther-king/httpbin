use std::io::Write;

use hyper::Get;
use hyper::header::{ContentLength, ContentType};
use hyper::mime::{Mime, SubLevel, TopLevel};
use hyper::server::{Request, Response};
use hyper::uri::RequestUri;

static INDEX: &'static [u8] = br#"<!doctype html>
<head>
  <title>Clone of httpbin.org with Rust</title>
</head>
<body>
  <ul>
    <li><a href="/">/</a> This page.</li>
  </ul>
</body>"#;

pub fn route(req: Request, mut res: Response) {
    let body = match req.uri {
        RequestUri::AbsolutePath(ref path) => {
            match (req.method, &path[..]) {
                (Get, "/") => INDEX,
                _ => b"",
            }
        }
        _ => b"",
    };

    res.headers_mut().set(ContentLength(body.len() as u64));
    res.headers_mut()
        .set(ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![])));

    let mut res = res.start().unwrap();
    res.write_all(body).unwrap();
}
