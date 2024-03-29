//! copyright © ecdata.cn 2021 - present

use super::ReplyRender;
use wechat_sdk::current_timestamp;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ImageReply {
    pub from_user: String,
    pub to_user: String,
    pub time: u64,
    pub media_id: String,
}

impl ImageReply {
    #[inline]
    pub fn new<S: Into<String>>(from_user: S, to_user: S, media_id: S) -> ImageReply {
        ImageReply {
            from_user: from_user.into(),
            to_user: to_user.into(),
            time: current_timestamp(),
            media_id: media_id.into(),
        }
    }
}

impl ReplyRender for ImageReply {
    #[inline]
    fn render(&self) -> String {
        format!(r#"<xml>
            <ToUserName><![CDATA[{to_user}]]></ToUserName>
            <FromUserName><![CDATA[{from_user}]]></FromUserName>
            <CreateTime>{time}</CreateTime>
            <MsgType><![CDATA[image]]></MsgType>
            <Image>
            <MediaId><![CDATA[{media_id}]]></MediaId>
            </Image>
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
    use super::ImageReply;

    #[test]
    fn test_render_image_reply() {
        let reply = ImageReply::new("test1", "test2", "test");
        let rendered = reply.render();
        assert!(rendered.contains("test1"));
        assert!(rendered.contains("test2"));
        assert!(rendered.contains("test"));
    }
}
