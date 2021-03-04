use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use wechat_sdk::{
    tripartite::{set_tripartite_config, TripartiteConfig},
    RedisConfig,
};
// 业务配置信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub name: String,
    pub port: i32,
    pub tripartite: Option<TripartiteConfig>,
<<<<<<< HEAD
    pub redisconfig: Option<RedisConfig>,
=======
    // pub weapp: Option<>
>>>>>>> 69b0773a5f90bd73dffa9ff6477dd26220c36b1c
}

impl Config {
    /// 加载配置
    pub fn new(config_path: &str) -> Self {
        let file_path = if config_path.is_empty() {
            "prod.conf"
        } else {
            config_path
        };

        // 打开文件
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("no such file {} exception: {}", file_path, e),
        };

        // 读取文件到字符串变量
        let mut str_val = String::new();
        match file.read_to_string(&mut str_val) {
            Ok(s) => s,
            Err(e) => panic!("Error Reading file:{}", e),
        };
        let cnf: Config = serde_json::from_str(&str_val).unwrap();
        // println!("{:?}", cnf);
        // 第三方配置处理
        match cnf.tripartite.clone() {
            Some(val) => {
                set_tripartite_config(val.clone());
                val
            }
            _ => {
                println!("请配置第三方文件!");
                TripartiteConfig::default()
            }
        };
        match cnf.redisconfig.clone() {
            Some(val) => val,
            _ => {
                println!("请配置redis缓存!");
                RedisConfig::default()
            }
        };
        cnf
    }
}
