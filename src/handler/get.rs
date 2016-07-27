use std::collections::HashMap;
use std::io::Write;

use hyper::header::{ContentLength, ContentType};
use hyper::mime::{Mime, SubLevel, TopLevel};
use hyper::server::{Request, Response};
use hyper::uri::RequestUri;

use handler::headers;
use handler::ip;

pub struct Get<'a> {
    args: HashMap<&'a str, Vec<String>>,
    headers: headers::HeaderCollection,
    origin: ip::Ip,
    url: String,
}

/// GETリクエストハンドラ
pub fn get_handler(req: Request, mut res: Response) {}

/// クエリパラメータをkey/value形式にパースする
pub fn parse_query(url: &str) -> HashMap<&str, Vec<String>> {
    let mut hm: HashMap<&str, Vec<String>> = HashMap::new();

    if let Some(query) = url.split('?').nth(1) {
        let pairs = query.split('&')
            .map(|pair| pair.split('='))
            .map(|kv| (kv.clone().nth(0), kv.clone().nth(1)));

        for (k, v) in pairs {
            match k {
                None => (),
                Some(key) => {
                    let args = hm.entry(key).or_insert(vec![]);
                    args.push(v.unwrap_or("").to_owned());
                }
            }
        }
    }

    hm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_query() {
        let query = parse_query("/path?f=foo&b=bar");

        assert_eq!(query["f"], vec!["foo"]);
        assert_eq!(query["b"], vec!["bar"]);
    }

    #[test]
    fn test_parse_query_multi_values() {
        let query = parse_query("/path?f=foo1&f=foo2");

        assert_eq!(query["f"], vec!["foo1", "foo2"]);
    }

    #[test]
    fn test_parse_query_without_parameter() {
        let query = parse_query("/path");

        assert_eq!(query.len(), 0);
    }

    #[test]
    fn test_parse_query_without_value() {
        let query = parse_query("/path?f=foo&f=&b=bar");

        assert_eq!(query["f"], vec!["foo", ""]);
        assert_eq!(query["b"], vec!["bar"]);
    }
}
