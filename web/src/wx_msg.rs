//! copyright
//!
//! 微信消息处理
//!

use super::utils;
use actix_web::client::Client;
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Result};
use std::collections::HashMap;
use url::Url;
use wechat_sdk::{
    current_timestamp,
    message::{KFService, Message, ReplyRender, TextReply},
    tripartite::{get_tripartite_config, Component, TripartiteConfig},
    WeChatCrypto,
};

/// 消息回复处理
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

/// 代理消息业务转发
pub async fn proxy_reply(
    app_id: &str,
    req: HttpRequest,
    body: web::Bytes
) -> Result<HttpResponse> {
    use crate::cluster::get_domain;
    use wechat_sdk::Client;
    let mut domain = get_domain(app_id.to_owned());

    if domain.is_empty() {
        domain = "http://366kmpf.com".to_owned();
    }

    // 创建一个可变的url地址
    let mut new_url = Url::parse(&format!("{}/WxCallback.axd", domain)).unwrap();
    new_url.set_query(req.uri().query());

    match Client::new().post_betyes(new_url.as_str(), body).await {
        Ok(res) => {
            return Ok(HttpResponse::build(StatusCode::OK)
                .content_type("text/html; charset=utf-8")
                .body(res))
        }
        Err(err) => println!("{:?}", err),
    };

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("success"))
}

/// 全网发布
pub async fn global_publish(
    dic: HashMap<String, String>,
    post_str: String,
) -> Result<HttpResponse> {
    logs!(format!("--- callback ---- {:?}, {:?}", dic, post_str));

    let nonce = utils::get_hash_value(&dic, "nonce");
    // 对获取的消息内容进行解密
    let conf: TripartiteConfig = get_tripartite_config();
    let c = WeChatCrypto::new(&conf.token, &conf.encoding_aes_key, &conf.app_id);

    // 对接收的消息进行解码判断
    if let Ok(decode_msg) = c.decrypt_message(&post_str, &dic) {
        // println!("=== decode message === {}", decode_msg);
        let msg = Message::parse(&decode_msg);
        let to_user = msg.get_to_user();

        // 全网发布时的测试用户
        if to_user == "gh_3c884a361561" || to_user == "gh_8dad206e9538" {
            match msg {
                Message::TextMessage(ref m) => {
                    // 公网发布的授权消息处理
                    if m.content.starts_with("QUERY_AUTH_CODE:") {
                        let auth_code = m.content.replace("QUERY_AUTH_CODE:", "");

                        let comp = Component::new(conf);
                        // 根据授权码获取公众号对应的accesstoken
                        match comp.query_auth(&auth_code).await {
                            Ok(v) => {
                                // v 是一个Json对象,从json对象中获取授权 authorizer_access_token
                                if v["authorization_info"].is_object() {
                                    let auth_access_token = match v["authorization_info"]
                                        ["authorizer_access_token"]
                                        .as_str()
                                    {
                                        Some(token) => token.to_string(),
                                        None => "".to_owned(),
                                    };
                                    let kf = KFService::new(&auth_access_token);

                                    kf.send(
                                        &m.from_user,
                                        &"text".to_string(),
                                        &format!("{}_from_api", auth_code),
                                    )
                                    .await;
                                }
                            }
                            Err(e) => logs!(format!("query auth_code error: {:?}", e)),
                        };
                    }
                    // 文本消息回复处理
                    else if m.content == "TESTCOMPONENT_MSG_TYPE_TEXT" {
                        let tr = TextReply::new(
                            &m.to_user,
                            &m.from_user,
                            &format!("{}_callback", &m.content),
                        );
                        let txt = tr.render();
                        logs!(format!(
                            "---- send TESTCOMPONENT_MSG_TYPE_TEXT xml :{}",
                            txt
                        ));
                        let timestamp = current_timestamp();
                        let encrypt_text = c.encrypt_message(&txt, timestamp, &nonce);

                        return Ok(HttpResponse::build(StatusCode::OK)
                            .content_type("text/html; charset=utf-8")
                            .body(encrypt_text.unwrap()));
                    }
                    //其他消息
                    else {
                        let tr = TextReply::new(
                            &m.to_user,
                            &m.from_user,
                            &format!("{}_callback", &m.content),
                        );
                        let txt = tr.render();
                        let timestamp = current_timestamp();
                        let encrypt_text = c.encrypt_message(&txt, timestamp, &nonce);
                        logs!(format!("---- send OTHER xml :{}", txt));
                        return Ok(HttpResponse::build(StatusCode::OK)
                            .content_type("text/xml; charset=utf-8")
                            .body(encrypt_text.unwrap()));
                    }
                }
                Message::EventMessage(ref m) => {
                    logs!(format!("**** EVENT *** {:?}", m));
                }
                Message::UnknownMessage(ref m) => {
                    logs!(format!("**** Unknown *** {:?}", m));
                }
            }
        } else {
            use super::wx_msg;
            return wx_msg::message_reply(&msg).await;
        }
    }

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("success"))
}
