//! copyright © ecdata.cn 2021 - present
//! 微信开发对接配置信息
//! created by shaipe 20210302

// use serde_derive::{ Serialize, Deserialize };



use wechat_sdk::WechatResult;
use std::fs::File;

/// 微们接口平台类型
// #[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PlatformType {
    OfficialAccount, // 公众号
    OpenPlatfrom,    // 开放平台
    MiniProgram,     // 小程序
}

/// 微信sdk配置
pub struct Config {
    pub app_id: String, // 应用id
    pub secret: String, // 密钥
    pub token: String,  // token,在接口配置时填写的token,用于sigine验证
    pub platform: PlatformType, // 配置的平台类型
                        // pub msg_type: MessageFormat,    // 消息格式
                        // pub encrypt_mode: EncryptMode   // 加密方式
}

impl Config {
    /// 加载yml配置文件
    pub fn load_yaml(conf_path: &str) -> WechatResult<Config> {
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
        let _yaml_doc = &docs[0];
        // get server value
        // let server = yaml_doc["weapp"].clone();

        Ok(Config::default())
    }
}

/// 默认配置项
impl Default for Config {
    fn default() -> Self {
        Config {
            app_id: String::new(),
            secret: String::new(),
            token: String::new(),
            platform: PlatformType::MiniProgram,
            // msg_type: MessageFormat::Json,
            // encrypt_mode: EncryptMode::Plaintext
        }
    }
}
