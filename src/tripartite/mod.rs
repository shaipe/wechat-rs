//! copyright
//! 微信第三方平台


pub mod ticket;
pub use ticket::{Ticket, get_ticket, set_ticket};

pub mod config;
pub use config::{TripartiteConfig,set_tripartite_config,get_tripartite_config};

mod comp;
pub use comp::Component;