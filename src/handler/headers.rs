use std::collections::HashMap;
use std::io::Write;

use hyper::header::{ContentType, ContentLength};
use hyper::mime::{Mime, SubLevel, TopLevel};
use hyper::server::{Request, Response};
use rustc_serialize::json;

#[derive(RustcEncodable)]
pub struct HeaderCollection {
    headers: HashMap<String, String>,
}

/// Header一覧ハンドラ
pub fn headers_handler(req: Request, mut res: Response) {
    let mime = Mime(TopLevel::Application, SubLevel::Json, vec![]);
    let mut hc = HeaderCollection { headers: HashMap::new() };

    for h in req.headers.iter() {
        hc.headers.insert(h.name().to_owned(), h.value_string());
    }

    let json = json::encode(&hc).unwrap();
    let body = json.as_bytes();

    res.headers_mut().set(ContentType(mime));
    res.headers_mut().set(ContentLength(body.len() as u64));

    let mut res = res.start().unwrap();

    res.write_all(body).unwrap();
}
