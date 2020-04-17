//! copyright
//! 事件消息
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct EventMessage {
    pub to_user: String,
    pub from_user: String,
    pub create_time: u64,
    pub msg_type: String,
    pub event: String,
}

impl EventMessage {
    pub fn to_xml(&self) -> String {
        format!(
            r#"<ToUserName><![CDATA[{to_user}]]></ToUserName>
        <FromUserName><![CDATA[{from_user}]]></FromUserName>
        <CreateTime>{create_time}</CreateTime>
        <MsgType><![CDATA[event]]></MsgType>
        <Content><![CDATA[{content}]]></Content>"#,
            to_user = self.to_user,
            from_user = self.from_user,
            create_time = 3434,
            content = "sdds"
        )
    }
}
