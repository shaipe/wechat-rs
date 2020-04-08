use wechat_sdk::tripartite::{get_ticket, set_ticket, Ticket};

use super::utils;

use actix_web::http;
use actix_web::http::{StatusCode};
use actix_web::{ web, Error, HttpRequest, HttpResponse, Result};
use wechat_sdk::{
    tripartite::{get_tripartite_config, TripartiteConfig, WechatComponent},
    WeChatResult,
};

/// 第三方ticket推送接收处理
#[post("/wx/ticket")]
pub async fn receive_ticket(
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse, Error> {
    // 获取地址栏参数
    let dic = utils::parse_query(req.query_string());
    // 获取post数据
    let post_str = utils::get_request_body(payload).await;
    println!("url_param: {:?} \n post_str: {:?}", req.query_string(), post_str);

    let config: TripartiteConfig = get_tripartite_config();
    if let Ok(t) = Ticket::parse_ticket(config, &post_str, dic) {
        let mut ticket = get_ticket();
        ticket.access_ticket = t;
        ticket.save("");
        set_ticket(ticket);
    };

    // 告诉服务器接收成功
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("success"))
}

/// 发起授权
#[get("/auth")]
async fn official_auth(req: HttpRequest) -> Result<HttpResponse> {
    let query = req.query_string();
    let dic = utils::parse_query(query);
    //随机数
    let base_query = utils::get_hash_value(&dic, "q");
    let app_type = match base64::decode(&base_query) {
        Ok(val) => {
            let s = String::from_utf8(val).unwrap();

            let arr: Vec<&str> = s.split("|").collect();
            println!("q={:?}", arr[3]);
            if arr.len() == 5 {
                arr[3].parse::<u32>().unwrap()
            } else {
                1
            }
        }
        Err(_) => 1,
    };
    let config: TripartiteConfig = get_tripartite_config();
    let mut ticket = get_ticket();
    let token = ticket.get_token(config.clone()).await;
    //println!("access_token={:?}", token);
    let c = WechatComponent::new(&config.app_id, &config.secret, &ticket.access_ticket);
    let code = c.create_preauthcode(&token).await;
    //println!("code={:?}", code);
    let path = c.component_login_page(
        &code.unwrap(),
        &format!("{}/official_auth_calback?q={}", config.domain, base_query),
        app_type,
    );
    //println!("path={:?}", path);
    Ok(HttpResponse::build(StatusCode::FOUND)
        .header(http::header::LOCATION, path)
        .body(""))
}

/// 公众号授权回调
#[get("official_auth_calback")]
async fn official_auth_calback(req: HttpRequest) -> Result<HttpResponse> {
    let query = req.query_string();
    let dic = utils::parse_query(query);
    //随机数
    let base_query = utils::get_hash_value(&dic, "q");
    let auth_code = utils::get_hash_value(&dic, "auth_code");
    let path = match base64::decode(&base_query) {
        Ok(val) => {
            let s = String::from_utf8(val).unwrap();

            let arr: Vec<&str> = s.split("|").collect();
            println!("q={:?}", arr[4]);
            if arr.len() == 5 {
                format!("{}?p={}&auth_code={}", arr[4], base_query, auth_code)
            } else {
                "".to_owned()
            }
        }
        Err(_) => "".to_owned(),
    };

    println!("path={:?}", path);
    // response
    Ok(HttpResponse::build(StatusCode::FOUND)
        .header(http::header::LOCATION, path)
        .body(""))
}

/// 获取第三方的token
#[post("fetch_component_token")]
async fn fetch_component_token(req: HttpRequest) -> Result<HttpResponse> {
    // 获取token
    let token = match req.head().headers.get("token") {
        Some(t) => t.to_str().unwrap().to_string(),
        None => "".to_owned(),
    };

    // token无效时直接返回空值
    if token.is_empty() {
        return Ok(HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body(""));
    }

    let config: TripartiteConfig = get_tripartite_config();
    let mut ticket = get_ticket();
    let token = ticket.get_token(config.clone()).await;
    

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(ticket.access_token))
}


/// 微信第三方消息回调处理
pub async fn callback(
    req: HttpRequest,
    path: web::Path<(String,)>,
    payload: web::Payload,
) -> Result<HttpResponse> {
    let dic = utils::parse_query(req.query_string());
    println!("{:?}", dic);
    // payload is a stream of Bytes objects
    let post_str = utils::get_request_body(payload).await;

    println!("{:?}", post_str);

    // //随机数
    // let nonce = utils::get_hash_value(&dic, "nonce");
    // if nonce.is_empty() {
    //     return Ok(HttpResponse::build(StatusCode::OK)
    //         .content_type("text/html; charset=utf-8")
    //         .body("error"));
    // }
    // //时间缀
    // let timestamp = utils::get_hash_value(&dic, "timestamp")
    //     .parse::<i64>()
    //     .unwrap();
    // //签名信息
    // let signature = utils::get_hash_value(&dic, "msg_signature");

    // use wechat_sdk::message::Message;
    // let config: TripartiteConfig = get_tripartite_config();
    // let t = Message::new(&config.token, &config.encoding_aes_key, &config.app_id);
    // let result: WeChatResult<String> = t.parse(&post_str, &signature, timestamp, &nonce);

    // println!("{:?}", result);

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(format!("Hello {}!", path.0)))
}
