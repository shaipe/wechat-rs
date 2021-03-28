use super::WechatResult;
use redis::{self, FromRedisValue, ToRedisArgs};
use std::collections::{BTreeMap, BTreeSet, HashMap};
pub trait SessionStruct {}

pub trait SessionStore: Clone {
    //根据cmd类型获取指定 key 值
    //简单类型（string,vec,int):get; 哈希:hgetall(btreemap); set(btreeset)：smembers
    fn get<K: AsRef<str>, T: FromRedisValue>(
        &self,
        key: K,
        cmd: K,
        default: Option<T>,
    ) -> Option<T>;
    //设置多个哈希btreemap
    fn hmsets<K: AsRef<str>, T: ToRedisArgs + Copy>(&self, map: BTreeMap<K, BTreeMap<T, T>>);
    //设置单个哈希btreemap
    fn hmset<K: AsRef<str>, T: ToRedisArgs>(&self, key: K, map: T);
    //删除
    fn del<K: AsRef<str>>(&self, key: K);
    //set(btreeset) 添加
    fn sadd<K: AsRef<str>, T: ToRedisArgs>(&self, key: K, value: T);
    //简单类型（string,vec,int) set，带过期时间
    fn set<K: AsRef<str>, T: ToRedisArgs>(&self, key: K, value: T, ttl: Option<usize>);
    //发布订阅
    fn sub(&self, func: fn(Option<BTreeMap<String, String>>));
    //使用锁并设置过期时间
    fn setnx<K: AsRef<str>>(&self,key:K,argv1:K,argv2:K)->Option<i32>;
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
    //根据cmd类型获取指定 key 值
    //简单类型（string,vec,int):get; 哈希:hgetall(btreemap); set(btreeset)：smembers
    fn get<K: AsRef<str>, T: FromRedisValue>(
        &self,
        key: K,
        cmd: K,
        default: Option<T>,
    ) -> Option<T> {
        let conn = self.client.get_connection();
        if conn.is_err() {
            return default;
        }
        let mut conn = conn.unwrap();
        let data = redis::cmd(cmd.as_ref()).arg(key.as_ref()).query(&mut conn);
        if data.is_err() {
            println!("data:is_err");
            return default;
        }
        if let Ok(value) = data {
            Some(value)
        } else {
            default
        }
    }
    //设置多个哈希btreemap
    fn hmsets<K: AsRef<str>, T: ToRedisArgs + Copy>(&self, map: BTreeMap<K, BTreeMap<T, T>>) {
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
    //设置单个哈希btreemap
    fn hmset<K: AsRef<str>, T: ToRedisArgs>(&self, key: K, map: T) {
        let conn = self.client.get_connection();
        if conn.is_err() {
            return;
        }
        let mut conn = conn.unwrap();
        match redis::cmd("HSET")
            .arg(key.as_ref())
            .arg(map)
            .query(&mut conn)
        {
            Ok(v) => v,
            Err(e) => {
                println!("{:?},{:?}", key.as_ref(), e);
            }
        };
    }
    //set(btreeset) 添加
    fn sadd<K: AsRef<str>, T: ToRedisArgs>(&self, key: K, value: T) {
        let key = key.as_ref();
        let conn = self.client.get_connection();
        if conn.is_err() {
            return;
        }
        let mut conn = conn.unwrap();
        match redis::pipe()
            .cmd("SADD")
            .arg(key)
            .arg(value)
            .query(&mut conn)
        {
            Ok(v) => v,
            Err(e) => {
                println!("{:?},{:?}", key, e);
            }
        }
    }
    //简单类型（string,vec,int) set，带过期时间
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
                .cmd("set")
                .set(key, value)
                .query(&mut conn)
                .unwrap_or(());
            println!("{:?}", v)
        }
    }
    //根据key删除
    fn del<K: AsRef<str>>(&self, key: K) {
        let conn = self.client.get_connection();
        if conn.is_err() {
            return;
        }
        let mut conn = conn.unwrap();
        match redis::pipe().del(key.as_ref()).ignore().query(&mut conn) {
            Ok(v) => v,
            Err(e) => {
                println!("{:?},{:?}", key.as_ref(), e);
            }
        };
    }
    //发布订阅
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
    fn setnx<K: AsRef<str>>(&self,key:K,argv1:K,argv2:K)->Option<i32>{
        let lua_scripts = redis::Script::new(r#"if redis.call('setnx',KEYS[1],ARGV[1]) == 1 then
            redis.call('expire',KEYS[1],ARGV[2]) return 1 else return 0 end"#);
           
        let conn = self.client.get_connection();
        if conn.is_err() {
            return None;
        }
        let mut conn = conn.unwrap();

        let result = script.key(key).arg(argv1).arg(argv2).invoke(&mut con);
        match result{
            Ok(v)=>{
                Some(v)
            },
            Err=>{
                None
            }
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
    let url = format!("redis://:abc123!%40%23%24@139.9.173.11:6379/9");
    println!("{:?}", url);
    match RedisStorage::from_url(url) {
        Ok(session) => {
            println!("测试btreemap");
            let mut hdic = BTreeMap::new();
            let def_hdic: BTreeMap<String, String> = BTreeMap::new();
            hdic.insert(2, 2);
            session.hmset("hello-btreemap", hdic);
            match session.get("hello-btreemap", "hgetall", Some(def_hdic)) {
                Some(s) => {
                    println!("hello-btreemap {:?}", s);
                }
                None => {}
            };
            println!("测试简单类型");
            session.set("hello", "18981772611", Some(1000));
            let v = String::from("");
            match session.get("hello", "get", Some(v)) {
                Some(s) => {
                    println!("cccc {:?}", s);
                }
                None => {}
            };

            println!("测试btreeset");
            let mut sdic = BTreeSet::new();
            let def_sdic: BTreeSet<String> = BTreeSet::new();
            sdic.insert("setset".to_owned());
            sdic.insert("setset2".to_owned());
            session.sadd("hello-btreeset", sdic);
            match session.get("hello-btreeset", "smembers", Some(def_sdic)) {
                Some(s) => {
                    println!("hello-btreeset {:?}", s);
                }
                None => {}
            };
        }
        Err(e) => {
            println!("error==={:?}", e);
        }
    }
    assert_eq!(1 + 1, 1);
}
