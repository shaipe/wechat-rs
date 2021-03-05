//! copyright © ecdata.cn 2021 - present
//! 微信消息处理


mod parser;
pub use parser::{parse_message, MessageParser};

mod text;
pub use text::TextMessage;

mod event;
pub use event::EventMessage;

mod unknown;
pub use unknown::UnknownMessage;

/// 消息回复
mod reply;
pub use reply::{Reply, ReplyRender, TextReply};

mod kf;
pub use kf::KFService;

/// 消息枚举
#[derive(Debug, Clone)]
pub enum Message {
    TextMessage(TextMessage),
    EventMessage(EventMessage),
    UnknownMessage(UnknownMessage),
}

impl Message {
    /// 解析并返回Message对象
    pub fn parse<S: AsRef<str>>(xml: S) -> Message {
        parse_message(xml)
    }

    /// 获取消息谁发送的
    pub fn get_from_user(&self) -> String {
        match *self {
            Message::TextMessage(ref msg) => msg.from_user.to_owned(),
            Message::EventMessage(ref msg) => msg.from_user.to_owned(),
            Message::UnknownMessage(ref msg) => msg.from_user.to_owned(),
        }
    }

    /// 获取消息发送给谁
    pub fn get_to_user(&self) -> String {
        match *self {
            Message::TextMessage(ref msg) => msg.to_user.to_owned(),
            Message::EventMessage(ref msg) => msg.to_user.to_owned(),
            Message::UnknownMessage(ref msg) => msg.to_user.to_owned(),
        }
    }
}

