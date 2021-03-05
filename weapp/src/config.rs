//! copyright © ecdata.cn 2021 - present
//! 小程序后端接口对接配置信息
//! created by shaipe 20210228

use std::fs::File;
use wechat_sdk::WechatResult;

/// 配置信息结构体
pub struct WeappConfig {
    // 应用id
    pub app_id: String,
    // 应用密钥
    pub secret: String,
    // 小程序名称
    pub name: String,
}

impl WeappConfig {
    /// 加载yml配置文件
    pub fn load_yaml(conf_path: &str) -> WechatResult<WeappConfig> {
        use yaml_rust::yaml;
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
        use std::io::Read;
        match f.read_to_string(&mut s) {
            Ok(s) => s,
            Err(e) => {
                return Err(error! {
                    code: 4004,
                    msg: format!("Error Reading file: {}", e)
                });
            }
        };
        // f.read_to_string(&mut s).unwrap(); // read file content to s
        // load string to yaml loader
        let docs = yaml::YamlLoader::load_from_str(&s).unwrap();
        // get first yaml hash doc
        let yaml_doc = &docs[0];
        // get server value
        let server = yaml_doc["weapp"].clone();

        Ok(WeappConfig::load_yaml_node(&server))
    }

    /// 根据yaml配置点进加载配置
    /// @param1: yaml 配置节点
    pub fn load_yaml_node(conf_node: &yaml_rust::yaml::Yaml) -> WeappConfig {
        WeappConfig {
            app_id: if let Some(s) = conf_node["app_id"].as_str() {
                s.to_owned()
            } else {
                "".to_owned()
            },
            name: if let Some(s) = conf_node["name"].as_str() {
                s.to_owned()
            } else {
                "".to_owned()
            },
            secret: if let Some(s) = conf_node["secret"].as_str() {
                s.to_owned()
            } else {
                "".to_owned()
            },
        }
    }
}

/// 默认实现
impl std::default::Default for WeappConfig {
    // 给定默认值
    fn default() -> WeappConfig {
        WeappConfig {
            name: String::new(),
            app_id: String::new(),
            secret: String::new(),
        }
    }
}
