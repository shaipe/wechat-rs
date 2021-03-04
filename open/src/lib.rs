//! copyright
//! 微信第三方平台

#[macro_use]
extern crate wechat_sdk;
// 此句一定不能少
use wechat_sdk::{RedisStorage, SessionStore, WechatError};

// #[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

pub mod ticket;
pub use ticket::{get_ticket, set_ticket, Ticket};

pub mod config;
pub use config::{get_tripartite_config, set_tripartite_config, TripartiteConfig};

mod comp;
pub use comp::Component;
