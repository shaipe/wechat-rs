//! copyright 
//! 


mod parser;
pub use parser::{parse_message, MessageParser};

mod text;
pub use text::TextMessage;

mod unknown;
pub use unknown::UnknownMessage;

/// 消息枚举
pub enum Message {
    TextMessage(TextMessage),
    UnknownMessage(UnknownMessage)
}


impl Message {

    /// 解析并返回Message对象
    pub fn parse<S: AsRef<str>>(xml: String) -> Message {
        parse_message(xml)
    }

    /// 获取消息谁发送的
    pub fn get_from_user(&self) -> String {
        match *self {
            Message::TextMessage(ref msg) => msg.from_user.to_owned(),
            Message::UnknownMessage(ref msg) => msg.from_user.to_owned(),
        }
    }

    /// 获取消息发送给谁
    pub fn get_to_user(&self) -> String {
        match *self {
            Message::TextMessage(ref msg) => msg.to_user.to_owned(),
            Message::UnknownMessage(ref msg) => msg.to_user.to_owned(),
        }
    }
}
