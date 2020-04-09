use wechat_sdk::tripartite::{get_ticket, set_ticket, Ticket};

use super::utils;

use actix_web::http;
use actix_web::http::{StatusCode};
use actix_web::{ web, Error, HttpRequest, HttpResponse, Result};
use wechat_sdk::{
    tripartite::{get_tripartite_config, TripartiteConfig, WechatComponent},
    WeChatResult,
};
use md5;
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
async fn auth_transfer(req: HttpRequest) -> Result<HttpResponse> {
    let query = req.query_string();
    let path=format!("/official_auth?{}",query);
    println!("cctiv={:?}",path);
    Ok(HttpResponse::build(StatusCode::OK)
    .content_type("text/html; charset=utf-8")
    .body(format!("<script>location.href='{}'</script>",path)))
}
#[get("/official_auth")]
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
    println!("path={:?}", path);
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
            let absolute_path=arr[4].to_lowercase();
            let absolute_path=absolute_path.replace("websupplier/social/wechatset.aspx", "WxComponent.axd");
            println!("q={:?}", absolute_path);
            if arr.len() == 5 {
                format!("{}?q={}&auth_code={}", absolute_path, base_query, auth_code)
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
    use percent_encoding::percent_decode;
    // 获取token
    let token = match req.head().headers.get("token") {
        Some(t) => t.to_str().unwrap().to_string(),
        None => "".to_owned(),
    };
    //解码
    let token = percent_decode(token.as_bytes())
    .decode_utf8()
    .unwrap();
  
    // token无效时直接返回空值
    if token.is_empty() {
        return response_error("token 为空");
    }
    let md5_value = match req.head().headers.get("md5") {
        Some(t) => t.to_str().unwrap().to_string(),
        None => "".to_owned(),
    };
    let token_md5=format!(
        "{:x}",
        md5::compute(format!("rwxkj:{}", token).as_bytes())
    );
   
    if(md5_value!=token_md5){
        return response_error("校验失败");
    }
    let config: TripartiteConfig = get_tripartite_config();
    let mut ticket = get_ticket();

    let token = ticket.get_token(config.clone()).await;
    
    if token.is_empty(){
        response_error("获取token为空，请检查ticket是否正确推送")
    }
    else{
       response_ok(&token)
    }

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
///返回ok
fn response_ok(content:&str) -> Result<HttpResponse>{
    
    let result=format!(r#"{{"Success":true,"Code":200,"Message":"","Content":"{}"}}"#,content);
    println!("ok={:?}",result);
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .body(result))
}
///返回error
fn response_error(msg:&str) -> Result<HttpResponse>{
   
    let result=format!(r#"{{"Success":false,"Code":500,"Message":"{}","Content":""}}"#,msg);
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .body(result))
}