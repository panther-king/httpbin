use std::collections::BTreeMap;
use std::io::Write;
use std::net::SocketAddr;

use hyper::header::{ContentLength, ContentType};
use hyper::mime::{Mime, SubLevel, TopLevel};
use hyper::server::{Request, Response};
use rustc_serialize::json::{Json, ToJson};

pub struct Ip {
    origin: String,
}

impl Ip {
    pub fn new(sock: &SocketAddr) -> Ip {
        Ip { origin: ip_addr(&sock) }
    }

    /// プロパティをJSONオブジェクトとして返す
    pub fn as_json(&self) -> Json {
        self.origin.to_json()
    }

    /// JSONのキーとなるプロパティ名を返す
    pub fn key(&self) -> String {
        "origin".to_owned()
    }
}

impl ToJson for Ip {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();

        d.insert(self.key(), self.as_json());
        Json::Object(d)
    }
}

/// IPアドレスハンドラ
pub fn ip_handler(req: Request, mut res: Response) {
    let mime = Mime(TopLevel::Application, SubLevel::Json, vec![]);
    let ip = Ip::new(&req.remote_addr);
    let json = ip.to_json().pretty().to_string();
    let body = json.as_bytes();

    res.headers_mut().set(ContentType(mime));
    res.headers_mut().set(ContentLength(body.len() as u64));

    let mut res = res.start().unwrap();

    res.write_all(body).unwrap();
}

/// IPアドレスをフォーマットする
pub fn ip_addr(sock: &SocketAddr) -> String {
    match *sock {
        SocketAddr::V6(addr) => {
            addr.ip()
                .segments()
                .iter()
                .map(|s| format!("{}", s))
                .collect::<Vec<String>>()
                .join(":")
        }
        SocketAddr::V4(addr) => {
            addr.ip()
                .octets()
                .iter()
                .map(|o| format!("{}", o))
                .collect::<Vec<String>>()
                .join(".")
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net;
    use rustc_serialize::json::ToJson;
    use super::*;

    fn ipv4_sock() -> net::SocketAddr {
        let v4 = net::Ipv4Addr::new(127, 0, 0, 1);
        let addr = net::IpAddr::V4(v4);

        net::SocketAddr::new(addr, 80)
    }

    fn ipv6_sock() -> net::SocketAddr {
        let v6 = net::Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);
        let addr = net::IpAddr::V6(v6);

        net::SocketAddr::new(addr, 80)
    }

    #[test]
    fn test_ipv4_addr() {
        let sock = ipv4_sock();

        assert_eq!(ip_addr(&sock), "127.0.0.1");
    }

    #[test]
    fn test_ipv6_addr() {
        let sock = ipv6_sock();

        assert_eq!(ip_addr(&sock), "0:0:0:0:0:0:0:1");
    }

    #[test]
    fn test_ip_key() {
        let sock = ipv4_sock();
        let ip = Ip::new(&sock);

        assert_eq!(ip.key(), "origin");
    }

    #[test]
    fn test_ip_as_json() {
        let sock = ipv4_sock();
        let ip = Ip::new(&sock);

        assert_eq!(ip.as_json().to_string(), r#""127.0.0.1""#);
    }

    #[test]
    fn test_ip_to_json() {
        let sock = ipv4_sock();
        let ip = Ip::new(&sock);

        assert_eq!(ip.to_json().to_string(), r#"{"origin":"127.0.0.1"}"#);
    }
}
