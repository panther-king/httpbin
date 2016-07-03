extern crate hyper;

pub mod router;

/// HTTPサーバの起動
pub fn dispatch(address: &str) {
    hyper::server::Server::http(address)
        .unwrap()
        .handle(router::route)
        .unwrap();
}
