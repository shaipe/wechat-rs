//! copyright © ecdata.cn 2021 - present
//! 文本消息处理

use super::MessageParser;
use wechat_sdk::xmlutil;

// use super::WechatMessage;

// <xml>
// <ToUserName><![CDATA[toUser]]></ToUserName>
// <FromUserName><![CDATA[fromUser]]></FromUserName>
// <CreateTime>1348831860</CreateTime>
// <MsgType><![CDATA[text]]></MsgType>
// <Content><![CDATA[this is a test]]></Content>
// <MsgId>1234567890123456</MsgId>
// </xml>

/// 文本消息
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TextMessage {
    pub to_user: String,
    pub from_user: String,
    pub create_time: u64,
    pub content: String,
    pub id: i64,
    pub raw: String,
}

/// 消息解析器实现
impl MessageParser for TextMessage {
    /// 微信消息类型定义
    type WeChatMessage = TextMessage;

    #[inline]
    fn from_xml(xml: &str) -> TextMessage {
        let package = xmlutil::parse(xml);
        let doc = package.as_document();
        let source = xmlutil::evaluate(&doc, "//xml/FromUserName/text()").string();
        let target = xmlutil::evaluate(&doc, "//xml/ToUserName/text()").string();
        let id = xmlutil::evaluate(&doc, "//xml/MsgId/text()").number() as i64;
        let time = xmlutil::evaluate(&doc, "//xml/CreateTime/text()").number() as u64;
        let content = xmlutil::evaluate(&doc, "//xml/Content/text()").string();
        TextMessage {
            from_user: source,
            to_user: target,
            id: id,
            create_time: time,
            content: content,
            raw: xml.to_owned(),
        }
    }

    /// 转换为json数据
    fn to_json(&self) -> serde_json::Value {
        json!({
            "msgType": "text",
            "toUser": self.to_user,
            "fromUser": self.from_user,
            "id": self.id,
            "createTime": self.create_time,
            "content": self.content
        })
    }
}
