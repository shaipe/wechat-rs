//! copyright
//! 微信第三方平台

#[macro_use]
extern crate wechat_sdk;

// 此句一定不能少
use wechat_sdk::WechatError;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

mod ticket;
pub use ticket::Ticket;

mod config;
pub use config::{
    get_tripartite_config, get_tripartite_config_mut, set_tripartite_config, Config as TripartiteConfig,
};

mod comp;
pub use comp::Component;

mod open;
pub use open::OpenAccount;

pub mod weapp;

#[cfg(test)]
mod tests {
    use crate::{get_tripartite_config, Component, TripartiteConfig};

    #[test]
    fn test_redis() {
        let tripart_config: TripartiteConfig = get_tripartite_config();
        // let redis_config: RedisConfig = get_redis_conf();
        let comp = Component::new(tripart_config.clone());
        let x =
            actix_rt::System::new().block_on(comp.fetch_authorizer_info("wx999317f16de96ce3", ""));
        println!("{:?}", x);
        assert_eq!(1 + 1, 2);
    }
}
