use std::collections::BTreeMap;
use std::io::Write;

use hyper::header::{ContentLength, ContentType, UserAgent as UA};
use hyper::mime::{Mime, SubLevel, TopLevel};
use hyper::server::{Request, Response};
use rustc_serialize::json::{Json, ToJson};

const UNKNOWN: &'static str = "Unknown User-Agent";

pub struct UserAgent {
    agent: String,
}

impl ToJson for UserAgent {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();

        d.insert("user-agent".to_owned(), self.agent.to_json());
        Json::Object(d)
    }
}

/// User-Agentハンドラ
pub fn user_agent_handler(req: Request, mut res: Response) {
    let mime = Mime(TopLevel::Application, SubLevel::Json, vec![]);
    let agent = match req.headers.get::<UA>() {
        Some(&UA(ref s)) => s.to_owned(),
        None => UNKNOWN.to_owned(),
    };
    let ua = UserAgent { agent: agent };
    let json = ua.to_json().to_string();
    let body = json.as_bytes();

    res.headers_mut().set(ContentType(mime));
    res.headers_mut().set(ContentLength(body.len() as u64));

    let mut res = res.start().unwrap();

    res.write_all(body).unwrap();
}

#[cfg(test)]
mod tests {
    use rustc_serialize::json::ToJson;
    use super::*;

    #[test]
    fn test_user_agent_to_json() {
        let ua = UserAgent { agent: "Firefox".to_owned() };
        let json = ua.to_json().to_string();

        assert_eq!(json, "{\"user-agent\":\"Firefox\"}");
    }
}
