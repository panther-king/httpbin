use std::io::Write;
use std::net::SocketAddr;

use hyper::header::{ContentLength, ContentType};
use hyper::mime::{Mime, SubLevel, TopLevel};
use hyper::server::{Request, Response};
use rustc_serialize::json;

#[derive(RustcEncodable)]
pub struct Ip {
    origin: String,
}

impl Ip {
    pub fn new(sock: SocketAddr) -> Ip {
        Ip { origin: ip_addr(sock) }
    }
}

/// IPアドレスハンドラ
pub fn ip_handler(req: Request, mut res: Response) {
    let mime = Mime(TopLevel::Application, SubLevel::Json, vec![]);
    let ip = Ip::new(req.remote_addr);
    let json = json::encode(&ip).unwrap();
    let body = json.as_bytes();

    res.headers_mut().set(ContentType(mime));
    res.headers_mut().set(ContentLength(body.len() as u64));

    let mut res = res.start().unwrap();

    res.write_all(body).unwrap();
}

/// IPアドレスをフォーマットする
pub fn ip_addr(sock: SocketAddr) -> String {
    match sock {
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
    use super::*;

    #[test]
    fn test_ipv4_addr() {
        let v4 = net::Ipv4Addr::new(127, 0, 0, 1);
        let addr = net::IpAddr::V4(v4);
        let sock = net::SocketAddr::new(addr, 80);

        assert_eq!(ip_addr(sock), "127.0.0.1");
    }

    #[test]
    fn test_ipv6_addr() {
        let v6 = net::Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);
        let addr = net::IpAddr::V6(v6);
        let sock = net::SocketAddr::new(addr, 80);

        assert_eq!(ip_addr(sock), "0:0:0:0:0:0:0:1");
    }
}
