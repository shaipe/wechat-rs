//! copyright © ecdata.cn 2021 - present
//! 事件消息

use super::MessageParser;
use wechat_sdk::xmlutil;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct EventMessage {
    pub to_user: String,
    pub from_user: String,
    pub create_time: i64,
    pub msg_type: String,
    pub event: String,
    pub raw: String,
}

/// 消息解析器实现
impl MessageParser for EventMessage {
    /// 微信消息类型定义
    type WeChatMessage = EventMessage;

    #[inline]
    fn from_xml(xml: &str) -> EventMessage {
        let package = xmlutil::parse(xml);
        let doc = package.as_document();
        let source = xmlutil::evaluate(&doc, "//xml/FromUserName/text()").string();
        let target = xmlutil::evaluate(&doc, "//xml/ToUserName/text()").string();
        // let id = xmlutil::evaluate(&doc, "//xml/MsgId/text()").number() as i64;
        let time = xmlutil::evaluate(&doc, "//xml/CreateTime/text()").number() as i64;
        let content = xmlutil::evaluate(&doc, "//xml/Event/text()").string();
        EventMessage {
            from_user: source,
            to_user: target,
            create_time: time,
            event: content,
            msg_type: "event".to_owned(),
            raw: xml.to_owned(),
        }
    }
}
