

use serde_derive::{ Serialize, Deserialize };

/// 消息格式
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageFormat {
    XML,
    Json,
}

impl Default for MessageFormat {
    fn default() -> Self {
        MessageFormat::XML
    }
}

/// 加密方式
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EncryptMode {
    Plaintext,  
    Hybrid,
    Encrypted
}

/// 给定默认加密模式
impl Default for EncryptMode {
    fn default() -> Self {
        EncryptMode::Plaintext
    }
}

/// 微们接口平台类型
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PlatformType {
    OfficialAccount,    // 公众号
    OpenPlatfrom,       // 开放平台
    MiniProgram,        // 小程序
}

/// 微信sdk配置
pub struct Config {
    pub app_id: String,     // 应用id
    pub secret: String,     // 密钥
    pub token: String,      // token,在接口配置时填写的token,用于sigine验证
    pub platform: PlatformType, // 配置的平台类型
    pub msg_type: MessageFormat,    // 消息格式 
    pub encrypt_mode: EncryptMode   // 加密方式
}

/// 默认配置项
impl Default for Config {

    fn default() -> Self {
        Config {
            app_id: String::new(),
            secret: String::new(),
            token: String::new(),
            platform: PlatformType::MiniProgram,
            msg_type: MessageFormat::Json,
            encrypt_mode: EncryptMode::Plaintext
        }
    }
}