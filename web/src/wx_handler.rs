use wechat_sdk::tripartite::{get_ticket, set_ticket, Ticket};

use super::utils;

use super::result_response::{
    get_exception_result, get_success_result, get_success_result2, ResultResponse,
};
use actix_web::http;
use actix_web::http::StatusCode;
use actix_web::{web, Error, HttpRequest, HttpResponse, Result};
use md5;
use std::collections::HashMap;
use wechat_sdk::{
    official::WechatAuthorize,
    tripartite::{get_tripartite_config, TripartiteConfig, Component},
    xmlutil, WeChatCrypto, WeChatResult,
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
    println!(
        "Ticket Request Start:  url_param: {:?} \n post_str: {:?}",
        req.query_string(),
        post_str
    );

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
    let path = format!("/official_auth?{}", query);
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(format!("<script>location.href='{}'</script>", path)))
}

/// 公众号授权
#[get("/official_auth")]
async fn official_auth(req: HttpRequest) -> Result<HttpResponse> {
    let query = req.query_string();
    let dic = utils::parse_query(query);
    //随机数
    let base_query = utils::get_hash_value(&dic, "q");
    println!("base_query={:?}", base_query);
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
    let c = Component::new(config.clone());

    let code = c.create_preauthcode(&token).await;

    println!("base_query={:?}", base_query);
    use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
    let base_query = utf8_percent_encode(&base_query, NON_ALPHANUMERIC).to_string();
    //println!("base_query={:?}",base_query);
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
    println!("sss{:?}", req.uri().host());
    //随机数
    let base_query = utils::get_hash_value(&dic, "q");
    let auth_code = utils::get_hash_value(&dic, "auth_code");
    use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
    let path = match base64::decode(&base_query) {
        Ok(val) => {
            let s = String::from_utf8(val).unwrap();

            let arr: Vec<&str> = s.split("|").collect();
            let absolute_path = arr[4].to_lowercase();
            let absolute_path =
                absolute_path.replace("websupplier/social/wechatset.aspx", "WxComponent.axd");
            let absolute_path =
                absolute_path.replace("webzone/social/wechatset.aspx", "WxComponent.axd");
            //println!("q={:?}", absolute_path);
            if arr.len() == 5 {
                format!(
                    "{}?q={}&auth_code={}",
                    absolute_path,
                    utf8_percent_encode(&base_query, NON_ALPHANUMERIC).to_string(),
                    auth_code
                )
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
    let token = percent_decode(token.as_bytes()).decode_utf8().unwrap();
    // token无效时直接返回空值
    if token.is_empty() {
        return get_exception_result("token 为空", 500);
    }
    let md5_value = match req.head().headers.get("md5") {
        Some(t) => t.to_str().unwrap().to_string(),
        None => "".to_owned(),
    };
    let token_md5 = format!("{:x}", md5::compute(format!("rwxkj:{}", token).as_bytes()));

    if (md5_value != token_md5) {
        return get_exception_result("校验失败", 500);
    }
    let config: TripartiteConfig = get_tripartite_config();
    let mut ticket = get_ticket();

    let token = ticket.get_token(config.clone()).await;

    if token.is_empty() {
        get_exception_result("获取token为空，请检查ticket是否正确推送", 500)
    } else {
        let mut content_dic: HashMap<String, String> = HashMap::new();
        content_dic.insert("token".to_owned(), token);
        content_dic.insert(
            "expires_in".to_owned(),
            format!("{}", ticket.at_expired_time),
        );
        get_success_result(&content_dic)
    }
}

/// 微信第三方消息回调处理
/// https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/Post_Application_on_the_Entire_Network/releases_instructions.html
/// 上面是全网发布的资料
pub async fn callback(
    req: HttpRequest,
    path: web::Path<(String,)>,
    payload: web::Payload,
) -> Result<HttpResponse> {
    use wechat_sdk::message::{Message, MessageParser, TextReply, ReplyRender};

    let dic = utils::parse_query(req.query_string());
    // println!("{:?}", dic);
    // payload is a stream of Bytes objects
    let post_str = utils::get_request_body(payload).await;

    println!("callback {:?}, {:?}", dic,  post_str);

    // 对获取的消息内容进行解密
    let conf: TripartiteConfig = get_tripartite_config();
    let c = WeChatCrypto::new(&conf.token, &conf.encoding_aes_key, &conf.app_id);
    match c.decrypt_message(&post_str, dic) {
        Ok(v) => {
            // println!("decode_msg: {:?}", v.clone());
            let msg = Message::parse(&v);
            let to_user = msg.get_to_user();

            // 全网发布时的测试用户
            if to_user == "gh_3c884a361561" || to_user == "gh_8dad206e9538" {
                match msg {
                    Message::TextMessage(ref m) => {
                        // 公网发布的授权消息处理
                        if m.content.starts_with("QUERY_AUTH_CODE:") {
                            let auth_code = m.content.replace("QUERY_AUTH_CODE:", "");
                            println!("auth code: {}", auth_code);
                            let config: TripartiteConfig = get_tripartite_config();
                            let comp = Component::new(config);
                            
                            // 根据授权码获取公众号对应的accesstoken
                            match comp.query_auth(&auth_code).await {
                                Ok(v) => {
                                    // v 是一个Json对象,从json对象中获取授权 authorizer_access_token
                                let auth_access_token = match v["authorizer_access_token"].as_str() {
                                    Some(token) => {
                                        token.to_string()
                                    }
                                    None => "".to_owned(),
                                };
                                    let kf = wechat_sdk::message::KFService::new(&auth_access_token);
                                    kf.send(&m.from_user, &"text".to_string(), &format!("{}_from_api", auth_code)).await;
                                    println!("{:?}", v);
                                },
                                Err(e) => println!("{:?}", e)
                            };
                        }
                        else{
                            let tr = TextReply::new(
                                &m.from_user,
                                &m.to_user,
                                &format!("{}_callback", &m.content),
                            );
                            // println!("{}", &m.content);
                            return Ok(HttpResponse::build(StatusCode::OK)
                                .content_type("application/xml; charset=utf-8")
                                .body(tr.render()));
                        }
                    }
                    Message::UnknownMessage(ref m) => {
                        println!("{:?}", m);
                    }
                }
            }
            // let ticketstr = xmlutil::evaluate(&doc, "//xml/ComponentVerifyTicket/text()").string();
            //Ok(ticketstr)
        }
        Err(e) => {
            println!("err: {}", e);
        }
    }
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
///获得授权url
#[post("fetch_auth_url")]
pub async fn fetch_auth_url(req: HttpRequest, payload: web::Payload) -> Result<HttpResponse> {
    let config: TripartiteConfig = get_tripartite_config();
    let query = req.query_string();
    let post_str = utils::get_request_body(payload).await;
    //println!("query={:?}",post_str);

    let dic = utils::parse_query(&post_str);
    //随机数
    let app_id = utils::get_hash_value(&dic, "app_id");
    let domain = if config.wap_domain.starts_with("http") {
        config.wap_domain
    } else {
        format!("http://{}", config.wap_domain)
    };
    let redirect_uri = format!("{}/user_auth_calback", &domain);
    let state = utils::get_hash_value(&dic, "state");

    let authorize = WechatAuthorize::new(&app_id, &config.app_id, "");
    let mut scopes = Vec::new();
    scopes.push("snsapi_userinfo");
    let url = authorize.get_authorize_url(&redirect_uri, &state, &scopes, "code");
    get_success_result2(&url)
}
/// 用户授权回调
#[get("user_auth_calback")]
async fn user_auth_calback(req: HttpRequest) -> Result<HttpResponse> {
    let query = req.query_string();
    let dic = utils::parse_query(query);
    //随机数
    let base_query = utils::get_hash_value(&dic, "state");
    let auth_code = utils::get_hash_value(&dic, "code");
    let path = match base64::decode(&base_query) {
        Ok(val) => {
            let s = String::from_utf8(val).unwrap();
            let arr: Vec<&str> = s.split("|").collect();
            if arr.len() == 3 {
                let hashQuery = arr[0];
                let fkway = arr[1];
                let back_domain = arr[2].to_lowercase();
                let state=base64::encode(&format!("{}|{}|",hashQuery,fkway));
                use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
                let state=utf8_percent_encode(&state,NON_ALPHANUMERIC).to_string();

                format!("{}/authback?code={}&state={}", back_domain, auth_code,state)
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
