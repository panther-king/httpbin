use hyper::server::{Request, Response};
use hyper::status::StatusCode;
use hyper::uri::RequestUri;

use handler;
use router::Router;

/// ルーティングを定義して、対応するハンドラを実行する
pub fn dispatch(req: Request, mut res: Response) {
    let mut router = Router::new();

    router.get("/", handler::index_handler);
    router.get("/ip", handler::ip_handler);
    router.get("/user-agent", handler::user_agent_handler);
    router.get("/headers", handler::headers_handler);

    match router.resolve(&req.method, parse(&req.uri)) {
        Some(ref r) => {
            let handler = r.handler();
            handler(req, res);
        }
        None => {
            let mut status = res.status_mut();
            *status = StatusCode::NotFound;
        }
    };
}

/// URIからリクエストパスを取り出す
pub fn parse(uri: &RequestUri) -> &str {
    match *uri {
        RequestUri::AbsolutePath(ref s) => s.split("?").nth(0).unwrap_or(""),
        _ => "",
    }
}

#[cfg(test)]
mod tests {
    use hyper::uri::RequestUri;
    use super::*;

    #[test]
    fn test_parse_with_path() {
        let uri = RequestUri::AbsolutePath("/hoge?foo=foo".to_owned());

        assert_eq!(parse(&uri), "/hoge");
    }

    #[test]
    fn test_parse_without_path() {
        let uri = RequestUri::AbsolutePath("?foo=foo".to_owned());

        assert_eq!(parse(&uri), "");
    }
}
