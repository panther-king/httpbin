use std::collections::HashMap;
use std::io::Write;

use hyper::header::{ContentLength, ContentType, Headers};
use hyper::mime::{Mime, SubLevel, TopLevel};
use hyper::server::{Request, Response};
use rustc_serialize::json;

#[derive(RustcEncodable)]
pub struct HeaderCollection {
    headers: HashMap<String, String>,
}

impl HeaderCollection {
    pub fn new(headers: Headers) -> HeaderCollection {
        HeaderCollection { headers: headers_map(headers) }
    }
}

/// Header一覧ハンドラ
pub fn headers_handler(req: Request, mut res: Response) {
    let mime = Mime(TopLevel::Application, SubLevel::Json, vec![]);
    let hc = HeaderCollection::new(req.headers);
    let json = json::encode(&hc).unwrap();
    let body = json.as_bytes();

    res.headers_mut().set(ContentType(mime));
    res.headers_mut().set(ContentLength(body.len() as u64));

    let mut res = res.start().unwrap();

    res.write_all(body).unwrap();
}

/// リクエストヘッダをkey/value形式に変換する
pub fn headers_map(headers: Headers) -> HashMap<String, String> {
    let mut hm = HashMap::new();

    for h in headers.iter() {
        hm.insert(h.name().to_owned(), h.value_string());
    }

    hm
}

#[cfg(test)]
mod tests {
    use hyper::header;
    use super::*;

    #[test]
    fn test_headers_map() {
        let mut headers = header::Headers::new();

        headers.set(header::Host {
            hostname: "example.com".to_owned(),
            port: None,
        });

        let vec = headers_map(headers);

        assert_eq!(vec.get("Host").unwrap(), "example.com");
        assert!(vec.get("User-Agent").is_none());
    }
}
