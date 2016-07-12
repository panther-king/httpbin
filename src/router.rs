use hyper::method::Method;
use hyper::server::{Request, Response};

pub type Handler = fn(Request, Response);

pub struct Route {
    method: Method,
    path: String,
    handler: Handler,
}

impl Route {
    pub fn handler(&self) -> &Handler {
        &self.handler
    }

    /// 自身のルーティングとマッチするか判定する
    pub fn is_match(&self, method: &Method, path: &str) -> bool {
        self.method == *method && self.path == path
    }
}

pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    /// Routerオブジェクトを生成する
    pub fn new() -> Router {
        Router { routes: Vec::new() }
    }

    /// DELETEメソッドのルーティングを追加する
    pub fn delete(&mut self, path: &str, handler: Handler) {
        self.add_route(Method::Delete, path, handler);
    }

    /// GETメソッドのルーティングを追加する
    pub fn get(&mut self, path: &str, handler: Handler) {
        self.add_route(Method::Get, path, handler);
    }

    /// POSTメソッドのルーティングを追加する
    pub fn post(&mut self, path: &str, handler: Handler) {
        self.add_route(Method::Post, path, handler);
    }

    /// PUTメソッドのルーティングを追加する
    pub fn put(&mut self, path: &str, handler: Handler) {
        self.add_route(Method::Put, path, handler);
    }

    /// 指定されたルーティングを解決する
    pub fn resolve(&self, method: &Method, path: &str) -> Option<&Route> {
        self.routes
            .iter()
            .find(|r| r.is_match(method, path))
    }

    /// ルーティングを追加する
    fn add_route(&mut self, method: Method, path: &str, handler: Handler) {
        let route = Route {
            method: method,
            path: path.to_owned(),
            handler: handler,
        };
        self.routes.push(route);
    }
}

#[cfg(test)]
mod tests {
    use hyper::method::Method;
    use hyper::server::{Request, Response};
    use super::*;

    fn mock_handler(_: Request, _: Response) {
        ()
    }

    #[test]
    fn test_route_is_match() {
        let r = Route {
            method: Method::Get,
            path: "/foo".to_owned(),
            handler: mock_handler,
        };

        assert!(r.is_match(&Method::Get, "/foo"));
        assert!(!r.is_match(&Method::Post, "/foo"));
    }

    #[test]
    fn test_router_delete() {
        let mut r = Router::new();
        r.delete("/delete", mock_handler);

        assert!(r.resolve(&Method::Delete, "/delete").is_some());
        assert!(r.resolve(&Method::Delete, "/foo").is_none());
        assert!(r.resolve(&Method::Get, "/delete").is_none());
    }

    #[test]
    fn test_router_get() {
        let mut r = Router::new();
        r.get("/get", mock_handler);

        assert!(r.resolve(&Method::Get, "/get").is_some());
        assert!(r.resolve(&Method::Get, "/foo").is_none());
        assert!(r.resolve(&Method::Post, "/get").is_none());
    }

    #[test]
    fn test_router_post() {
        let mut r = Router::new();
        r.post("/post", mock_handler);

        assert!(r.resolve(&Method::Post, "/post").is_some());
        assert!(r.resolve(&Method::Post, "/foo").is_none());
        assert!(r.resolve(&Method::Get, "/post").is_none());
    }

    #[test]
    fn test_router_put() {
        let mut r = Router::new();
        r.put("/put", mock_handler);

        assert!(r.resolve(&Method::Put, "/put").is_some());
        assert!(r.resolve(&Method::Put, "/foo").is_none());
        assert!(r.resolve(&Method::Get, "/put").is_none());
    }

    #[test]
    fn test_router_multi_route() {
        let mut r = Router::new();
        r.get("/route1", mock_handler);
        r.get("/route2", mock_handler);

        assert!(r.resolve(&Method::Get, "/route1").is_some());
        assert!(r.resolve(&Method::Get, "/route2").is_some());
        assert!(r.resolve(&Method::Get, "/foo").is_none());
    }
}
