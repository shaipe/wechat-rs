//! copyright
//! 用于记录微信appid与域名的对应关系

use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use std::fs::File;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Official {
    pub appid: String,
    pub authorizer_access_token: String,
    pub authorizer_refresh_token: String,
    pub expires_in: i64,
}
impl Official {
    pub fn default() -> Self {
        Official {
            appid: String::from(""),
            authorizer_access_token: String::from(""),
            authorizer_refresh_token: String::from(""),
            expires_in: 0,
        }
    }
    /// 加载配置
    pub fn new(config_path: &str) -> Self {
        let file_path = if config_path.is_empty() {
            "official.conf"
        } else {
            config_path
        };

        // 打开文件
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(_e) =>{
                return Official::default();
            },
        };

        // 读取文件到字符串变量
        let mut str_val = String::new();
        match file.read_to_string(&mut str_val) {
            Ok(s) => s,
            Err(e) => {
                println!("Error Reading file:{}", e);
                return Official::default();
            }
        };

        let cnf = serde_json::from_str(&str_val);

        match cnf {
            Ok(val) => {
                let t: Official = val;
                set_common_official(t.clone());
                t
            }
            Err(e) => {
                println!("Official文件配置错误! {:?}", e);
                Official::default()
            }
        }
    }

    /// 保存Official到文件
    pub fn save(&self, config_path: &str) {
        let file_path = if config_path.is_empty() {
            "official.conf"
        } else {
            config_path
        };
        // 打开文件
        let mut file = match File::create(file_path) {
            Ok(f) => f,
            Err(e) =>panic!("no such file {} exception: {}", file_path, e),
        };

        // 读取文件到字符串变量
        let str_val = json!(self).to_string();
        // println!("path={:?}", str_val);
        match file.write_all(str_val.as_bytes()) {
            Ok(s) => {
                set_common_official(self.clone());
                s
            }
            Err(e) => panic!("Error Saving file:{}", e),
        };
    }
}

// 默认加载静态全局
lazy_static! {
    ///通用公众号
    pub static ref COMMON_OFFICIAL:Arc<Mutex<Official>> = Arc::new(Mutex::new(Official::default()));
}

// 设置缓存对象
pub fn set_common_official(dict: Official) {
    let conf = Arc::clone(&COMMON_OFFICIAL);
    let mut cache = conf.lock().unwrap();
    *cache = dict;
}

/// 设置配置信息到缓存中
pub fn get_common_official() -> Official {
    let conf = Arc::clone(&COMMON_OFFICIAL);
    let cache = conf.lock().unwrap();
    cache.clone()
}

#[cfg(test)]
mod tests {
    // use crate::official::Official;
    // #[test]
    // fn add() {
    //     let mut conf = Official::new("");
    //     conf.appid = "wxf9d78c09d2efa1bc".to_owned();
    //     conf.authorizer_access_token="33_UZSk5mAUZ_Wx2K-gNOgHFGfJm8hY6dTv2MPN55Il-5R0uxsxYs5ydhJQysKH_FTBvlHJRCKwzIlZDbdK2jZnt-N6qUtwT_zyv-vwJqFYNg-y33luXaYN0-wrWj33iK4QnKCFWAWiI5wS8pBJRBXiADDZJF".to_owned();
    //     conf.authorizer_refresh_token ="refreshtoken@@@oDqmetXh9-gfmGIACZbjDO6-rP5AWrf_rUYAPTjSWn4".to_owned();
        
    //     println!("{:?}",conf);
    //     conf.save("");
    // }
}
