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

/// IPアドレスハンドラ
pub fn ip_handler(req: Request, mut res: Response) {
    let mime = Mime(TopLevel::Application, SubLevel::Json, vec![]);
    let octets = match req.remote_addr {
        SocketAddr::V4(sock) => sock.ip().octets(),
        _ => [0, 0, 0, 0],
    };
    let address = octets.into_iter()
        .map(|o| format!("{}", o))
        .collect::<Vec<String>>()
        .join(".");
    let ip = Ip { origin: address };
    let json = json::encode(&ip).unwrap();
    let body = json.as_bytes();

    res.headers_mut().set(ContentType(mime));
    res.headers_mut().set(ContentLength(body.len() as u64));

    let mut res = res.start().unwrap();

    res.write_all(body).unwrap();
}
