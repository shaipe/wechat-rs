//! copyright © ecdata.cn 2021 - present
//! 消息回复

mod text;
pub use text::TextReply;

mod image;
pub use image::ImageReply;

mod articles;
pub use articles::ArticlesReply;

/// 消息回复
pub trait ReplyRender {
    /// 返回回复内容
    fn render(&self) -> String;
}

pub enum Reply {
    TextReply(TextReply),
    ImageReply(ImageReply),
    ArticlesReply(ArticlesReply),
}

impl Reply {
    pub fn render(&self) -> String {
        let reply = match *self {
            Reply::TextReply(ref r) => r.render(),
            Reply::ImageReply(ref r) => r.render(),
            // Reply::VoiceReply(ref r) => r.render(),
            // Reply::VideoReply(ref r) => r.render(),
            // Reply::MusicReply(ref r) => r.render(),
            Reply::ArticlesReply(ref r) => r.render(),
            // Reply::TransferCustomerServiceReply(ref r) => r.render(),
        };
        reply
    }
}
