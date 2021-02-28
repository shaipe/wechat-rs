

use crate::{xmlutil, current_timestamp};
use super::MessageParser;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct UnknownMessage {
    pub from_user: String,
    pub to_user: String,
    pub time: i64,
    pub create_time: i64,
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
        let time = xmlutil::evaluate(&doc, "//xml/CreateTime/text()").number() as i64;
        UnknownMessage {
            from_user: source,
            to_user: target,
            id: id,
            time: time,
            create_time: current_timestamp(),
            raw: xml.to_owned(),
        }
    }
}
