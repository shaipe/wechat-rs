 const APP_TICKET_CACHES: &str = "APP_TICKET_CACHES";
const COMP_CATCHE_KEY: &str = "COMP_ACCESS_TOKEN_CATCHE_KEY";
use wechat_redis::{RedisStorage, SessionStore,RedisConfig};
use wechat_sdk::{WechatResult};
pub struct RedisCache {
    redis_con: String,
}
impl RedisCache {
    pub fn new(redis_conf: RedisConfig) -> Self {
        let redis_con = format!(
            "redis://:{}@{}:{}/{}",
            &redis_conf.password, &redis_conf.server, &redis_conf.port, redis_conf.dbid
        );
        RedisCache {
            redis_con: redis_con,
        }
    }
    /// 批量设置
    pub fn set_ticket_cache(&self, key: &str, v: String) {
        let cache_key = format!("{0}_{1}", APP_TICKET_CACHES, key);
        match RedisStorage::from_url(self.redis_con.clone()) {
            Ok(session) => {
                session.set(cache_key, v, Some(10 * 60 * 60));
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
    /// 获取
    pub fn get_ticket_cache(&self, key: &str) -> String {
        let cache_key = format!("{}_{}", APP_TICKET_CACHES, key);
        let d = "".to_owned();
        match RedisStorage::from_url(self.redis_con.clone()) {
            Ok(session) => {
                if let Some(v) = session.get(cache_key, "get".to_owned(), None) {
                    v
                } else {
                    d
                }
            }
            Err(_) => d,
        }
    }
    /// 设置单个
    pub fn set_comp_token(&self, key: &str, cnf: (String, u64)) {
        let url = format!("{}", &self.redis_con);
        let cache_key = format!("{0}_{1}", COMP_CATCHE_KEY, key);
        match RedisStorage::from_url(url) {
            Ok(session) => {
                session.set(cache_key, format!("{}|{}", cnf.0, cnf.1), Some(2 * 55 * 60));
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }

    /// 获取
    pub fn get_comp_token(&self, key: &str) -> WechatResult<(String, u64)> {
        let cache_key = format!("{0}_{1}", COMP_CATCHE_KEY, key);
        match RedisStorage::from_url(format!("{}", &self.redis_con)) {
            Ok(session) => {
                let d = "".to_owned();
                if let Some(v) = session.get(cache_key, "get".to_owned(), Some(d)) {
                    let arr: Vec<_> = v.split('|').collect();
                    if arr.len() == 2 {
                        return Ok((arr[0].to_string(), arr[1].parse::<u64>().unwrap()));
                    }
                    Err(error! {code:600,msg:"数据不准确"})
                } else {
                    Err(error! {code:600,msg:"数据不准确"})
                }
            }
            Err(e) => {
                let msg = format!("{:?}", e);
                Err(error! {code:600,msg:msg})
            }
        }
    }
}
