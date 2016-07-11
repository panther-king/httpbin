use hyper::server::{Request, Response};
use hyper::uri::RequestUri;

use handler;
use router::Router;

/// ルーティングを定義して、対応するハンドラを実行する
pub fn dispatch(req: Request, res: Response) {
    let mut router = Router::new();
    router.get("/", handler::index);

    match router.resolve(&req.method, parse(&req.uri)) {
        None => (),
        Some(ref r) => {
            let handler = r.handler();
            handler(req, res);
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
