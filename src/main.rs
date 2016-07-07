extern crate hyper;

use std::io::Write;
use hyper::server::{Server, Request, Response};

fn main() {
    fn index(_: Request, res: Response) {
        let mut res = res.start().unwrap();
        res.write_all(b"").unwrap();
    }

    Server::http("0.0.0.0:8888")
        .unwrap()
        .handle(index)
        .unwrap();
}
