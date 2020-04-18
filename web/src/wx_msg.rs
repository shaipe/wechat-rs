//! copyright
//!
//! 微信消息处理
//!

use actix_web::http::StatusCode;
use actix_web::{web, Error, HttpRequest, HttpResponse, Result};
use wechat_sdk::message::{Message, ReplyRender, TextReply};

pub async fn message_reply(msg: &Message) -> Result<HttpResponse> {
    match msg {
        Message::TextMessage(ref m) => {
            println!("$$$$$$$$$$$$$$ text message $$$$$$$$$$$$$$\n{:?}", m);
            // let tr = TextReply::new(
            //     &m.to_user,
            //     &m.from_user,
            //     &format!("{}_callback", &m.content),
            // );
            // let txt = tr.render();
            // let timestamp = current_timestamp();
            // let encrypt_text = c.encrypt_message(&txt, timestamp, &nonce);
            // println!("---- send text reply xml :{}", txt);
            // return Ok(HttpResponse::build(StatusCode::OK)
            //     .content_type("text/xml; charset=utf-8")
            //     .body(encrypt_text.unwrap()));
        }
        Message::EventMessage(ref m) => {}
        Message::UnknownMessage(ref m) => {}
    }

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("success"))
}
