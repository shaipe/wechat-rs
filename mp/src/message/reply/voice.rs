//! copyright © ecdata.cn 2021 - present
//! 语音消息回复

use super::ReplyRender;
use crate::current_timestamp;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct VoiceReply {
    pub from_user: String,
    pub to_user: String,
    pub time: i64,
    pub media_id: String,
}

impl VoiceReply {
    #[inline]
    pub fn new<S: Into<String>>(from_user: S, to_user: S, media_id: S) -> VoiceReply {
        VoiceReply {
            from_user: from_user.into(),
            to_user: to_user.into(),
            time: current_timestamp(),
            media_id: media_id.into(),
        }
    }
}

impl ReplyRender for VoiceReply {
    #[inline]
    fn render(&self) -> String {
        format!(r#"<xml>
            <ToUserName><![CDATA[{to_user}]]></ToUserName>
            <FromUserName><![CDATA[{from_user}]]></FromUserName>
            <CreateTime>{time}</CreateTime>
            <MsgType><![CDATA[voice]]></MsgType>
            <Voice>
            <MediaId><![CDATA[{media_id}]]></MediaId>
            </Voice>
            </xml>"#,
            to_user=self.to_user,
            from_user=self.from_user,
            time=self.time,
            media_id=self.media_id
        )
    }
}

#[cfg(test)]
mod tests {
    use super::ReplyRender;
    use super::VoiceReply;

    #[test]
    fn test_render_voice_reply() {
        let reply = VoiceReply::new("test1", "test2", "test");
        let rendered = reply.render();
        assert!(rendered.contains("test1"));
        assert!(rendered.contains("test2"));
        assert!(rendered.contains("test"));
    }
}
