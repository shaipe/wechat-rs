use super::WechatResult;
use redis::{self, Commands, FromRedisValue, ToRedisArgs};
use std::collections::{BTreeMap, HashMap};
pub trait SessionStruct {}

pub trait SessionStore: Clone {
    fn get<K: AsRef<str>, T: FromRedisValue>(&self, key: K, default: Option<T>) -> Option<T>;
    fn set<K: AsRef<str>, T: ToRedisArgs>(&self, key: K, value: T, ttl: Option<usize>);
    fn del<K: AsRef<str>>(&self, key: K);
    fn sub(&self, func: fn(Option<BTreeMap<String, String>>));
    fn set_hashs<K: AsRef<str>, T: ToRedisArgs + Copy>(&self, map: BTreeMap<K, BTreeMap<T, T>>);
    fn set_hash<K: AsRef<str>, T: ToRedisArgs + Copy>(&self, key: K, map: BTreeMap<T, T>);
}

#[derive(Debug, Clone)]
pub struct RedisStorage {
    pub client: redis::Client,
}

impl RedisStorage {
    pub fn new(client: redis::Client) -> RedisStorage {
        RedisStorage { client: client }
    }

    pub fn from_url<U: AsRef<str>>(url: U) -> WechatResult<RedisStorage> {
        let key = url.as_ref();
        let mut hash = get_redis_client();
        if hash.contains_key(key) {
            match hash.get_mut(key) {
                Some(v) => {
                    return Ok(RedisStorage { client: v.clone() });
                }
                None => {}
            };
        }
        let client = match redis::Client::open(url.as_ref()) {
            Ok(c) => {
                println!("打开redis");
                hash.insert(key.to_string(), c.clone());
                set_redis_client(hash);
                c
            }
            Err(e) => return Err(error!(-1, format!("redis error: {}", e))),
        };
        Ok(RedisStorage { client: client })
    }
}

impl SessionStore for RedisStorage {
    fn get<K: AsRef<str>, T: FromRedisValue>(&self, key: K, default: Option<T>) -> Option<T> {
        let conn = self.client.get_connection();
        if conn.is_err() {
            return default;
        }
        let mut conn = conn.unwrap();
        let data = conn.hgetall(key.as_ref());
        if data.is_err() {
            return default;
        }
        if let Ok(value) = data {
            Some(value)
        } else {
            default
        }
    }
    fn set_hashs<K: AsRef<str>, T: ToRedisArgs + Copy>(&self, map: BTreeMap<K, BTreeMap<T, T>>) {
        let conn = self.client.get_connection();
        if conn.is_err() {
            return;
        }
        let mut conn = conn.unwrap();
        // redis::pipe().set_multiple(map);
        println!("{}", map.len());
        let mut pip = redis::pipe();
        for (key, h) in map.iter() {
            for (f, val) in h {
                pip.cmd("HSET")
                    .arg(key.as_ref())
                    .arg(f.clone())
                    .arg(val.clone());
            }
        }
        match pip.query(&mut conn) {
            Ok(v) => v,
            Err(_) => {}
        }
    }
    fn set_hash<K: AsRef<str>, T: ToRedisArgs + Copy>(&self, key: K, map: BTreeMap<T, T>) {
        let conn = self.client.get_connection();
        if conn.is_err() {
            return;
        }
        let mut conn = conn.unwrap();
        println!("{}", map.len());
        for (f, val) in map {
            match redis::cmd("HSET")
                .arg(key.as_ref())
                .arg(f.clone())
                .arg(val.clone())
                .query(&mut conn)
            {
                Ok(v) => v,
                Err(e) => {
                    println!("{:?},{:?}", key.as_ref(), e);
                }
            }
        }
    }
    fn set<K: AsRef<str>, T: ToRedisArgs>(&self, key: K, value: T, ttl: Option<usize>) {
        let key = key.as_ref();
        let conn = self.client.get_connection();
        if conn.is_err() {
            return;
        }
        let mut conn = conn.unwrap();
        if let Some(seconds) = ttl {
            let _: () = redis::pipe()
                .set_ex(key, value, seconds)
                .ignore()
                .query(&mut conn)
                .unwrap_or(());
        } else {
            let v: () = redis::pipe()
                .cmd("SADD")
                .set(key, value)
                .query(&mut conn)
                .unwrap_or(());
            println!("{:?}", v)
        }
    }

    fn del<K: AsRef<str>>(&self, key: K) {
        let conn = self.client.get_connection();
        if conn.is_err() {
            return;
        }
        let mut conn = conn.unwrap();
        let _: () = redis::pipe()
            .del(key.as_ref())
            .ignore()
            .query(&mut conn)
            .unwrap_or(());
    }
    fn sub(&self, func: fn(Option<BTreeMap<String, String>>)) {
        let conn = self.client.get_connection();
        if conn.is_err() {
            return;
        }
        let mut conn = conn.unwrap();

        let mut pubsub = conn.as_pubsub();
        let sub = pubsub.subscribe("soul_account");
        if sub.is_err() {
            return;
        }
        use std::thread;
        use std::time::Duration as StdDuration;
        let interval: u64 = 10;
        loop {
            let msg = match pubsub.get_message() {
                Ok(s) => s,
                Err(e) => {
                    println!("{:?}", e);
                    thread::sleep(StdDuration::new(interval, 0));
                    continue;
                }
            };

            let payload: Vec<u8> = match msg.get_payload() {
                Ok(s) => s,
                Err(e) => {
                    println!("cccc{:?}", e);
                    thread::sleep(StdDuration::new(interval, 0));
                    continue;
                }
            };
            println!("channel '{}': {:?}", msg.get_channel_name(), payload);
            let stream =
                serde_json::Deserializer::from_slice(&payload).into_iter::<serde_json::Value>();
            for value in stream {
                println!("{:?}", value);
            }
            // use std::mem;
            // unsafe {
            //     let foobar =
            //         mem::transmute::<Vec<u8>, std::collections::BTreeMap<String, String>>(payload);
            //     println!("{:?}", foobar);
            // }
            // let post_str = match std::collections::BTreeMap::new(&payload) {
            //     Ok(s) => s,
            //     Err(_e) => "",
            // };
            //func(Some(payload));
            thread::sleep(StdDuration::new(interval, 0));
        }
    }
}
use std::sync::{Arc, Mutex};
// 默认加载静态全局
lazy_static! {
    //加载状态
    pub static ref REDIS_CLIENT_CACHES: Arc<Mutex<HashMap<String,redis::Client>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

/// 设置
pub fn set_redis_client(cnf: HashMap<String, redis::Client>) {
    let counter = Arc::clone(&REDIS_CLIENT_CACHES);
    let mut cache = counter.lock().unwrap();
    *cache = cnf;
}

/// 获取
pub fn get_redis_client() -> HashMap<String, redis::Client> {
    let counter = Arc::clone(&REDIS_CLIENT_CACHES);
    let cache = counter.lock().unwrap();
    cache.clone()
}
/// 宏测试
#[test]
fn test_err() {
    use super::{RedisStorage, SessionStore};
    let url = format!("redis://127.0.0.1:6379/1");
    match RedisStorage::from_url(url) {
        Ok(session) => {
            println!("执行了set");
            session.set("hello", "18981772611", Some(10000));
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
    assert_eq!(1 + 1, 1);
}
