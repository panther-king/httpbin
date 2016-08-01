use std::collections::BTreeMap;
use std::io::Write;

use hyper::header::{ContentLength, ContentType, Host};
use hyper::mime::{Mime, SubLevel, TopLevel};
use hyper::net::HttpStream;
use hyper::server::{Request, Response};
use hyper::uri::RequestUri;
use openssl::ssl::SslStream;
use rustc_serialize::json::{Json, ToJson};

use handler::headers;
use handler::ip;

pub struct Get {
    args: BTreeMap<String, Vec<String>>,
    headers: headers::HeaderCollection,
    origin: ip::Ip,
    url: String,
}

impl Get {
    pub fn new(req: Request) -> Get {
        Get {
            args: parse_query(&req.uri),
            headers: headers::HeaderCollection::new(&req.headers),
            origin: ip::Ip::new(&req.remote_addr),
            url: absolute_url(&req),
        }
    }
}

impl ToJson for Get {
    fn to_json(&self) -> Json {
        let mut query = BTreeMap::new();

        for (k, v) in &self.args {
            let value = match v.len() {
                1 => v[0].to_owned().to_json(),
                _ => v.to_json(),
            };

            query.insert(k.to_owned(), value);
        }

        let mut args = BTreeMap::new();

        args.insert("args".to_owned(), query.to_json());
        args.insert(self.headers.key(), self.headers.as_json());
        args.insert(self.origin.key(), self.origin.as_json());
        args.insert("url".to_owned(), self.url.to_json());
        Json::Object(args)
    }
}

/// GETリクエストハンドラ
pub fn get_handler(req: Request, mut res: Response) {
    let mime = Mime(TopLevel::Application, SubLevel::Json, vec![]);
    let get = Get::new(req);
    let json = get.to_json().to_string();
    let body = json.as_bytes();

    res.headers_mut().set(ContentType(mime));
    res.headers_mut().set(ContentLength(body.len() as u64));

    let mut res = res.start().unwrap();

    res.write_all(body).unwrap();
}

/// リクエスト情報から完全なURLを生成する
pub fn absolute_url(req: &Request) -> String {
    let scheme = match req.ssl::<SslStream<HttpStream>>() {
        Some(_) => "https",
        None => "http",
    };
    let domain = match req.headers.get::<Host>() {
        Some(h) => h.hostname.to_owned(),
        None => "".to_owned(),
    };
    let parameter = match req.uri {
        RequestUri::AbsolutePath(ref s) => s.to_owned(),
        _ => unreachable!(),
    };

    format!("{}://{}{}", scheme, domain, parameter)
}

/// クエリパラメータをkey/value形式にパースする
pub fn parse_query(uri: &RequestUri) -> BTreeMap<String, Vec<String>> {
    let mut hm: BTreeMap<String, Vec<String>> = BTreeMap::new();

    match *uri {
        RequestUri::AbsolutePath(ref s) => {
            if let Some(query) = s.split('?').nth(1) {
                let pairs = query.split('&')
                    .map(|pair| pair.split('='))
                    .map(|kv| (kv.clone().nth(0), kv.clone().nth(1)));

                for (k, v) in pairs {
                    match k {
                        None => (),
                        Some(key) => {
                            let args = hm.entry(key.to_owned()).or_insert(vec![]);
                            args.push(v.unwrap_or("").to_owned());
                        }
                    }
                }
            }
        }
        _ => (),
    }

    hm
}

#[cfg(test)]
mod tests {
    use hyper::uri::RequestUri::AbsolutePath;
    use super::*;

    #[test]
    fn test_parse_query() {
        let uri = AbsolutePath("/path?f=foo&b=bar".to_owned());
        let query = parse_query(&uri);

        assert_eq!(query["f"], vec!["foo"]);
        assert_eq!(query["b"], vec!["bar"]);
    }

    #[test]
    fn test_parse_query_multi_values() {
        let uri = AbsolutePath("/path?f=foo1&f=foo2".to_owned());
        let query = parse_query(&uri);

        assert_eq!(query["f"], vec!["foo1", "foo2"]);
    }

    #[test]
    fn test_parse_query_without_parameter() {
        let uri = AbsolutePath("/path".to_owned());
        let query = parse_query(&uri);

        assert_eq!(query.len(), 0);
    }

    #[test]
    fn test_parse_query_without_value() {
        let uri = AbsolutePath("/path?f=foo&f=&b=bar".to_owned());
        let query = parse_query(&uri);

        assert_eq!(query["f"], vec!["foo", ""]);
        assert_eq!(query["b"], vec!["bar"]);
    }
}
