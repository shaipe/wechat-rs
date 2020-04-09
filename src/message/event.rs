//! copyright
//! 事件消息

pub struct EventMessage {
    pub to_user_name: String,
    pub from_user_name: String,
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
            to_user = self.to_user_name,
            from_user = self.from_user_name,
            create_time = 3434,
            content = "sdds"
        )
    }
}
