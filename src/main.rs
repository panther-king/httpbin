extern crate hyper;

use std::io::Write;

use hyper::header::{ContentLength, ContentType};
use hyper::mime::{Mime, SubLevel, TopLevel};
use hyper::server::{Request, Response, Server};

fn main() {
    fn index_handler(_: Request, mut res: Response) {
        let body = r#"<!doctype html>
<html>
  <head>
    <title>Clone of httpbin.org with Rust</title>
  </head>
  <body>
    <ul>
      <li><a href="/">/</a> This page.</li>
    </ul>
  </body>
</html>"#
            .as_bytes();

        res.headers_mut().set(ContentLength(body.len() as u64));
        res.headers_mut().set(ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![])));

        let mut res = res.start().unwrap();
        res.write_all(body).unwrap();
    }

    Server::http("0.0.0.0:8888")
        .unwrap()
        .handle(index_handler)
        .unwrap();
}
