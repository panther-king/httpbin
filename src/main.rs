extern crate hyper;
extern crate httpbin;

use hyper::server::Server;
use httpbin::dispatcher::dispatch;

fn main() {
    Server::http("0.0.0.0:8888")
        .unwrap()
        .handle(dispatch)
        .unwrap();
}
