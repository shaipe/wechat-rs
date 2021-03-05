//! copyright © ecdata.cn 2021 - present
//! 文本消息回复

use wechat_sdk::current_timestamp;
use super::ReplyRender;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TextReply {
    pub from_user: String,
    pub to_user: String,
    pub time: i64,
    pub content: String,
}

impl TextReply {
    #[inline]
    pub fn new<S: Into<String>>(from_user: S, to_user: S, content: S) -> TextReply {
        TextReply {
            from_user: from_user.into(),
            to_user: to_user.into(),
            time: current_timestamp(),
            content: content.into(),
        }
    }
}

impl ReplyRender for TextReply {
    #[inline]
    fn render(&self) -> String {
        format!(
            "<xml>\n\
            <ToUserName><![CDATA[{to_user}]]></ToUserName>\n\
            <FromUserName><![CDATA[{from_user}]]></FromUserName>\n\
            <CreateTime>{time}</CreateTime>\n\
            <MsgType><![CDATA[text]]></MsgType>\n\
            <Content><![CDATA[{content}]]></Content>\n\
            </xml>",
            to_user = self.to_user,
            from_user = self.from_user,
            time = self.time,
            content = self.content
        )
    }
}
