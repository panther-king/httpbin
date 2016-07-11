use std::io::Write;

use hyper::header::{ContentLength, ContentType};
use hyper::mime::{Mime, SubLevel, TopLevel};
use hyper::server::{Request, Response};

const TOP: &'static str = r#"<!doctype html>
<html>
  <head>
    <meta charset="utf-8">
    <title>httpbin.org clone with Rust</title>
  </head>
  <body>
    <ul>
      <li><a href="/"><code>/</code></a> This page.</li>
    </ul>
  </body>
</html>
"#;

pub fn index(_: Request, mut res: Response) {
    let mime = Mime(TopLevel::Text, SubLevel::Html, vec![]);
    let body = TOP.as_bytes();

    res.headers_mut().set(ContentType(mime));
    res.headers_mut().set(ContentLength(body.len() as u64));

    let mut res = res.start().unwrap();

    res.write_all(body).unwrap();
}
