//! copyright © ecdata.cn 2021 - present
//! 微信开发对接配置信息
//! created by shaipe 20210302

// use serde_derive::{ Serialize, Deserialize };



use crate::WechatResult;
use std::fs::File;
use once_cell::sync::OnceCell;
// 默认加载静态全局
static CONFIGS: OnceCell<Config> = OnceCell::new();

/// 微们接口平台类型
// #[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(Debug, Clone)]
pub enum PlatformType {
    OfficialAccount, // 公众号
    OpenPlatfrom,    // 开放平台
    MiniProgram,     // 小程序
}


/// 微信sdk配置
#[derive(Debug, Clone)]
pub struct Config {
    pub app_id: String, // 应用id
    pub secret: String, // 密钥
    pub token: String,  // token,在接口配置时填写的token,用于sigine验证
    pub platform: PlatformType, // 配置的平台类型
                        // pub msg_type: MessageFormat,    // 消息格式
                        // pub encrypt_mode: EncryptMode   // 加密方式
    pub mch_id: String, //商户id
    pub private_key: String, //商户证书私钥
    pub certificate: String, //商户证书路径
    pub secret_key: String,  //API 秘钥
    
}

impl Config {

    /// 设置配置
    pub fn load(params: serde_json::Value) -> WechatResult<Config> {
        match CONFIGS.get() {
            Some(conf) => {
                Ok(conf.clone())
            },
            None => {
                //保存值
                let conf = Config{
                    app_id: format!("{}",params["app_id"].as_str().unwrap_or_default()),
                    secret: format!("{}",params["secret"].as_str().unwrap_or_default()),
                    token: format!("{}",params["token"].as_str().unwrap_or_default()),
                    platform: PlatformType::MiniProgram,
                    mch_id: format!("{}",params["mch_id"].as_str().unwrap_or_default()),
                    private_key: format!("{}",params["private_key"].as_str().unwrap_or_default()),
                    certificate: format!("{}",params["certificate"].as_str().unwrap_or_default()),
                    secret_key: format!("{}",params["secret_key"].as_str().unwrap_or_default())
                };
                let _ = CONFIGS.set(conf.clone());

                Ok(conf)
            }
        }
        
    }

    /// 获取对应参数 
    pub fn get() -> Config {
        match CONFIGS.get() {
            Some(conf) => {
                conf.clone()
            },
            None => {
                Config::default()
            }
        }
    }

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
            mch_id: "".to_string(),
            private_key: "".to_string(),
            certificate: "".to_string(),
            secret_key: "".to_string(),
            // msg_type: MessageFormat::Json,
            // encrypt_mode: EncryptMode::Plaintext
        }
    }
}
