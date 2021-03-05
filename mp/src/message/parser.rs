//! copyright © ecdata.cn 2021 - present
//! 

use wechat_sdk::xmlutil;
use super::{Message, TextMessage, UnknownMessage};


/// 消息解析器
pub trait MessageParser {
    type WeChatMessage;

    fn from_xml(xml: &str) -> Self::WeChatMessage;
}



/// 解析Message信息
pub fn parse_message<S: AsRef<str>>(xml: S) -> Message {
    let xml = xml.as_ref();
    let package = xmlutil::parse(xml);
    let doc = package.as_document();
    let msg_type_str = xmlutil::evaluate(&doc, "//xml/MsgType/text()").string().to_lowercase();
    let msg_type = &msg_type_str[..];
    let msg = match msg_type {
        "text" => Message::TextMessage(TextMessage::from_xml(xml)),
        // "image" => Message::ImageMessage(messages::ImageMessage::from_xml(xml)),
        // "voice" => Message::VoiceMessage(messages::VoiceMessage::from_xml(xml)),
        // "shortvideo" => Message::ShortVideoMessage(messages::ShortVideoMessage::from_xml(xml)),
        // "video" => Message::VideoMessage(messages::VideoMessage::from_xml(xml)),
        // "location" => Message::LocationMessage(messages::LocationMessage::from_xml(xml)),
        // "link" => Message::LinkMessage(messages::LinkMessage::from_xml(xml)),
        // "event" => {
        //     let event_str = xmlutil::evaluate(&doc, "//xml/Event/text()").string().to_lowercase();
        //     if &event_str == "subscribe" {
        //         let event_key = xmlutil::evaluate(&doc, "//xml/EventKey/text()").string();
        //         if &event_key != "" {
        //             // special SubscribeScanEvent
        //             return Message::SubscribeScanEvent(messages::SubscribeScanEvent::from_xml(xml));
        //         }
        //     }
        //     parse_event(&event_str[..], xml)
        // },
        _ => Message::UnknownMessage(UnknownMessage::from_xml(xml)),
    };
    msg
}

// fn parse_event(event: &str, xml: &str) -> Message {
//     match event {
//         "subscribe" => Message::SubscribeEvent(messages::SubscribeEvent::from_xml(xml)),
//         "unsubscribe" => Message::UnsubscribeEvent(messages::UnsubscribeEvent::from_xml(xml)),
//         "scan" => Message::ScanEvent(messages::ScanEvent::from_xml(xml)),
//         "location" => Message::LocationEvent(messages::LocationEvent::from_xml(xml)),
//         "click" => Message::ClickEvent(messages::ClickEvent::from_xml(xml)),
//         "view" => Message::ViewEvent(messages::ViewEvent::from_xml(xml)),
//         "qualification_verify_success" => Message::QualificationVerifySuccessEvent(messages::QualificationVerifySuccessEvent::from_xml(xml)),
//         _ => Message::UnknownMessage(messages::UnknownMessage::from_xml(xml)),
//     }
// }