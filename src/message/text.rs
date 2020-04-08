//! copyright
//! 文本消息处理


// <xml>
// <ToUserName><![CDATA[toUser]]></ToUserName>
// <FromUserName><![CDATA[fromUser]]></FromUserName>
// <CreateTime>1348831860</CreateTime>
// <MsgType><![CDATA[text]]></MsgType>
// <Content><![CDATA[this is a test]]></Content>
// <MsgId>1234567890123456</MsgId>
// </xml>

pub struct TextMessage {
    pub to_user_name: String,
    pub from_user_name: String,
    pub create_time: u64,
    pub msg_type: String,
    pub content: String,
    pub msg_id: u64
}


impl TextMessage {

}