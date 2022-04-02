use std::time::{SystemTime, UNIX_EPOCH};
use wechat::{
    open::{get_tripartite_config, Component, TripartiteConfig},
};
use wechat_redis::{get_redis_conf, RedisConfig};
use super::cache::RedisCache;

/// 获取第三方access_token
pub async fn get_comp_access_tokens() -> (String, i64) {
    let tripart_config: TripartiteConfig = get_tripartite_config();
    let redis_config: RedisConfig = get_redis_conf();
    let redis_cache = RedisCache::new(redis_config.clone());
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let token = match redis_cache.get_comp_token(&tripart_config.app_id) {
        Ok(s) => s,
        Err(_) => ("".to_owned(), 0),
    };
    let expires_at: i64 = token.1;
    //比较过期时间
    if expires_at <= timestamp || token.0.len() == 0 {
        let comp = Component::new(tripart_config.clone());
        let ticket = redis_cache.get_ticket_cache(&tripart_config.app_id);
        let result = comp.fetch_access_token(ticket).await;

        match result {
            Ok(token) => {
                redis_cache.set_comp_token(&tripart_config.app_id ,token.clone());
                token
            },
            Err(_) => ("".to_owned(), 0),
        }
    } else {
        token
    }
}