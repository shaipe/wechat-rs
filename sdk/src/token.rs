//! copyright © ecdata.cn 2021 - present
//! 接口凭证管理，直接使用文件进行管理
//! created by shaipe 20211012

use crate::WechatResult as Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Mutex;
use yaml_rust::{
    yaml::{Hash, Yaml},
    YamlEmitter, YamlLoader,
};

// 配置文件路径
const CONFIG_PATH: &'static str = "conf/wechat.yml";

// 缓存key
const SMS_CONFIG_KEY: &'static str = "wechat_config_cache";

// 默认加载静态全局
lazy_static! {
    pub static ref SMS_CONFIG_CACHES: Mutex<HashMap<String, AccessToken>> =
        Mutex::new(HashMap::new());
}

/// Access Token对象
#[derive(Debug, Clone, Default)]
pub struct AccessToken {
    // 应用类型
    pub app_type: String,
    // 访问token
    pub access_token: String,
    // access_token获取时间
    pub create_time: i64,
    // 有效期
    pub expires: i64,
}

impl AccessToken {
    /// 创建一个短信配置实例
    pub fn new() -> Result<AccessToken> {
        AccessToken::load_yaml(CONFIG_PATH)
    }

    pub fn load(file_path: &str) -> Self {
        AccessToken {
            app_type: "weapp".to_owned(),
            access_token: "".to_owned(),
            create_time: 0,
            expires: 0,
        }
    }

    /// 获取一个配置的新实例，并从指定的配置中加载配置信息
    /// @param1: 配置文件路径
    pub fn load_yaml(conf_path: &str) -> Result<AccessToken> {
        match AccessToken::load_string(conf_path) {
            Ok(s) => {
                let docs = YamlLoader::load_from_str(&s).unwrap();
                // get first yaml hash doc
                let yaml_doc = &docs[0];
                log!("load_yaml == {:?}", yaml_doc);

                let cnf = AccessToken {
                    app_type: if let Some(s) = yaml_doc["app_type"].as_str() {
                        s.to_owned()
                    } else {
                        "".to_owned()
                    },
                    access_token: if let Some(s) = yaml_doc["access_token"].as_str() {
                        s.to_owned()
                    } else {
                        "".to_owned()
                    },
                    create_time: if let Some(s) = yaml_doc["create_time"].as_i64() {
                        s
                    } else {
                        0
                    },
                    expires: if let Some(s) = yaml_doc["expires"].as_i64() {
                        s.to_owned()
                    } else {
                        0
                    },
                };
                Ok(cnf)
            }
            Err(err) => return Err(err),
        }
    }

    /// 加载配置文件的字符串
    /// @param1: 配置文件路径
    fn load_string(conf_path: &str) -> Result<String> {
        // open file
        let mut f = match File::open(conf_path) {
            Ok(f) => f,
            Err(e) => {
                return Err(error! {
                    code: 4004,
                    msg: format!("{}", e)
                });
            }
        };
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap(); // read file content to s

        Ok(s)
    }

    /// 把字符串对象写入缓存中,并指定有有效期单位秒
    pub fn set(val: AccessToken) {
        let key = SMS_CONFIG_KEY;
        // log!("setting config");
        SMS_CONFIG_CACHES
            .lock()
            .unwrap()
            .insert(key.to_owned(), val);
        // log!("setted config");
    }

    /// 获取cache中的缓存数据
    pub fn get() -> Option<AccessToken> {
        let key = SMS_CONFIG_KEY;
        let cache = SMS_CONFIG_CACHES.lock().unwrap();

        if let Some(cnf) = cache.get(key) {
            return Some(cnf.clone());
        }
        // else {
        //     match AccessToken::load_yaml() {
        //         Ok(c) => {
        //             Some(c)
        //         }
        //         Err(err) => {
        //             log!("sms config get error {}", err);
        //             None
        //         }
        //     }
        // }
        None
    }

    /// 保存修改后的配置信息
    pub fn save(&mut self) -> Result<bool> {
        let mut doc = Hash::new();
        doc.insert(
            Yaml::String("access_token".into()),
            Yaml::String(self.access_token.clone()),
        );
        doc.insert(
            Yaml::String("app_type".into()),
            Yaml::String(self.app_type.clone()),
        );
        doc.insert(
            Yaml::String("create_time".into()),
            Yaml::Integer(self.create_time),
        );
        doc.insert(Yaml::String("expires".into()), Yaml::Integer(self.expires));

        let mut out_str = String::new();
        {
            let mut emitter = YamlEmitter::new(&mut out_str);
            emitter.dump(&Yaml::Hash(doc)).unwrap(); // dump the YAML object to a String
        }

        // println!("{}", out_str);
        crate::write_to_file(CONFIG_PATH, out_str)

        // Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut conf = AccessToken::default();
        // conf.username = "dlxumin1".to_owned();
        // conf.password = "dlxumin1123".to_owned();
        // conf.sign = "宏推".to_owned();
        let _ = conf.save();
        println!("test");
    }
}
