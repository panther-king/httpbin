pub use self::headers::headers_handler;
pub use self::index::index_handler;
pub use self::ip::ip_handler;
pub use self::user_agent::user_agent_handler;

mod headers;
mod index;
mod ip;
mod user_agent;
