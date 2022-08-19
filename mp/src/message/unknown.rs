//! copyright © ecdata.cn 2021 - present
//! 未知消息类型

use super::MessageParser;
use wechat_sdk::xmlutil;

/// 未知类型的消息结构体
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct UnknownMessage {
    pub from_user: String,
    pub to_user: String,
    pub create_time: u64,
    pub id: i64,
    pub raw: String,
}

impl MessageParser for UnknownMessage {
    type WeChatMessage = UnknownMessage;

    #[inline]
    fn from_xml(xml: &str) -> UnknownMessage {
        let package = xmlutil::parse(xml);
        let doc = package.as_document();
        let source = xmlutil::evaluate(&doc, "//xml/FromUserName/text()").string();
        let target = xmlutil::evaluate(&doc, "//xml/ToUserName/text()").string();
        let id = xmlutil::evaluate(&doc, "//xml/MsgId/text()").number() as i64;
        let time = xmlutil::evaluate(&doc, "//xml/CreateTime/text()").number() as u64;
        UnknownMessage {
            from_user: source,
            to_user: target,
            id: id,
            create_time: time,
            raw: xml.to_owned(),
        }
    }

    fn to_json(&self) -> serde_json::Value {
        json!({
            "msgType": "unknown",
            "toUser": self.to_user,
            "fromUser": self.from_user,
            "id": self.id,
            "createTime": self.create_time
        })
    }
}
