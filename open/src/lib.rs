//! copyright
//! 微信第三方平台

#[macro_use]
extern crate wechat_sdk;

// 此句一定不能少
use wechat_sdk::{WechatError, WechatResult};

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

mod ticket;
pub use ticket::Ticket;

mod config;
pub use config::{get_tripartite_config, get_tripartite_config_mut, set_tripartite_config, Config};

mod token;
pub use token::AuthToken;

mod open;
pub use open::OpenAccount;

pub mod weapp;

// 定义接口请求域名
pub(crate) const API_DOMAIN: &'static str = "https://api.weixin.qq.com";

// 需要刷新AccessToken
const REFETCH_ACCESS_TOKEN_ERRCODES: [i32; 3] = [40001, 40014, 42001];

/// 解析post请求结果
pub(crate) async fn parse_json(res: &str) -> WechatResult<serde_json::Value> {
    let data = match wechat_sdk::json_decode(&res) {
        Ok(_data) => _data,
        Err(err) => {
            use wechat_sdk::ErrorKind;
            if let ErrorKind::Custom { code, .. } = err.kind {
                if REFETCH_ACCESS_TOKEN_ERRCODES.contains(&code) {
                    return Err(err);
                } else {
                    return Err(err);
                }
            } else {
                return Err(err);
            }
        }
    };
    Ok(data)
}

#[cfg(test)]
mod tests {
    use crate::{get_tripartite_config, AuthToken, Config as TripartiteConfig};

    #[test]
    fn test_redis() {
        let tripart_config: TripartiteConfig = get_tripartite_config();
        // let redis_config: RedisConfig = get_redis_conf();
        let comp = AuthToken::new(tripart_config.clone());
        let x =
            actix_rt::System::new().block_on(comp.fetch_authorizer_info("wx999317f16de96ce3", ""));
        println!("{:?}", x);
        assert_eq!(1 + 1, 2);
    }
}
