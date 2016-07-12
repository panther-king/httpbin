use std::io::Write;

use hyper::header::{ContentLength, ContentType};
use hyper::mime::{Mime, SubLevel, TopLevel};
use hyper::server::{Request, Response};

/// INDEXページHTML
const TOP: &'static str = r#"<!doctype html>
<html>
  <head>
    <meta charset="utf-8">
    <title>httpbin.org clone with Rust</title>
  </head>
  <body>
    <ul>
      <li><a href="/"><code>/</code></a> This page.</li>
      <li><a href="/ip"><code>/ip</code></a> Returns Origin IP.</li>
      <li><a href="/user-agent"><code>/user-agent</code></a> Returns user-agent.</li>
    </ul>
  </body>
</html>
"#;

/// INDEXページハンドラ
pub fn index_handler(_: Request, mut res: Response) {
    let mime = Mime(TopLevel::Text, SubLevel::Html, vec![]);
    let body = TOP.as_bytes();

    res.headers_mut().set(ContentType(mime));
    res.headers_mut().set(ContentLength(body.len() as u64));

    let mut res = res.start().unwrap();

    res.write_all(body).unwrap();
}
