//! copyright
//! 用于记录微信appid与域名的对应关系

use serde_json::json;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};

use wechat_sdk::{get_redis_conf, RedisStorage, SessionStore};
/// 从缓存中获取配置信息
pub fn get_domain(key: String) -> String {
    let cache = get_domains();
    let v = match cache.get(&key) {
        Some(val) => val.clone(),
        _ => "".to_owned(),
    };
    return v;
}

/// 添加域名
pub fn add_domain(key: String, val: String) {
    let mut domains: BTreeMap<String, String> = get_domains();
    domains.insert(key, val);
    // 写入文件存储
    write_clusters("", domains.clone());
    // 设置缓存
    set_domains(domains);
}

/// 写入文件
fn write_clusters(config_path: &str, content: BTreeMap<String, String>) {
    let file_path = if config_path.is_empty() {
        "domains.conf"
    } else {
        config_path
    };

    // 打开文件
    let mut file = match File::create(file_path) {
        Ok(f) => f,
        Err(e) => panic!("no such file {} exception: {}", file_path, e),
    };

    // 读取文件到字符串变量
    let str_val = json!(content).to_string();
    // println!("path={:?}", str_val);
    match file.write_all(str_val.as_bytes()) {
        Ok(s) => s,
        Err(e) => panic!("Error Reading file:{}", e),
    };
}

/// 加载
#[allow(dead_code)]
pub fn load_cluster(config_path: &str) {
    let file_path = if config_path.is_empty() {
        "domains.conf"
    } else {
        config_path
    };

    // 如果没有配置ticket文件,返回默认值
    if !std::path::Path::new(file_path).exists() {
        return;
    }

    // 打开文件
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            println!("no such file {} exception: {}", file_path, e);
            return;
        }
    };

    // 读取文件到字符串变量
    let mut str_val = String::new();
    match file.read_to_string(&mut str_val) {
        Ok(s) => s,
        Err(e) => {
            println!("Error Reading file:{}", e);
            return;
        }
    };
    // println!("{}", str_val);

    let domains: BTreeMap<String, String> = serde_json::from_str(&str_val).unwrap();
    // 加载出来后写入缓存
    set_domains(domains);
}

// 默认加载静态全局
lazy_static! {
    pub static ref APP_DOMAIN_CACHES: Arc<Mutex<HashMap<String, String>>> =
        Arc::new(Mutex::new(HashMap::new()));
}
const SALF_DOMAIN_CATCHE_KEY: &str = "SALF_DOMAIN_CATCHE_KEY";
// 设置缓存对象
pub fn set_domains(dict: BTreeMap<String, String>) {
    // let domains = Arc::clone(&APP_DOMAIN_CACHES);
    // let mut cache = domains.lock().unwrap();
    // *cache = dict;

    let redisconfig = get_redis_conf();
    let pwd: String = form_urlencoded::Serializer::new(redisconfig.password).finish();
    let url = format!(
        "{}:{}:{}/{}",
        redisconfig.server, redisconfig.port, pwd, redisconfig.dbid
    );
    match RedisStorage::from_url(url) {
        Ok(session) => {
            session.hmset(SALF_DOMAIN_CATCHE_KEY, dict);
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}

/// 设置配置信息到缓存中
pub fn get_domains() -> BTreeMap<String, String> {
    // let domains = Arc::clone(&APP_DOMAIN_CACHES);
    // let cache = domains.lock().unwrap();
    // cache.clone()
    let redisconfig = get_redis_conf();
    let pwd: String = form_urlencoded::Serializer::new(redisconfig.password).finish();
    let url = format!(
        "{}:{}:{}/{}",
        redisconfig.server, redisconfig.port, pwd, redisconfig.dbid
    );
    let obj = BTreeMap::new();
    match RedisStorage::from_url(url) {
        Ok(session) => {
            if let Some(v) = session.get(SALF_DOMAIN_CATCHE_KEY, "hgetall", None) {
                v
            } else {
                obj
            }
        }
        Err(e) => {
            println!("{:?}", e);
            obj
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    // use std::collections::HashMap;
    #[test]
    fn load() {
        // let mut x:HashMap<String, String> = HashMap::new();
        // x.insert("wxf9d78c09d2efa1bc".to_owned(), "http://b2b3231a.366ec.net".to_owned());
        // x.insert("wxf9d78c09d2efa1ba".to_owned(), "http://b2b3231a.366ec.net".to_owned());
        // x.insert("wxf9d78c09d2efa1bc".to_owned(), "http://b2b3231a.366ec.net".to_owned());
        // write_clusters("", x);
        load_cluster("");
        assert_eq!(1 + 1, 3);
    }
}
