//! copyright © ecdata.cn 2021 - present
//! 公众号中使用到的枚举定义

use std::default::Default;

/// 消息格式
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageFormat {
    XML,
    Json,
}

/// 默认实现
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