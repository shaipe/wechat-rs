//! copyright
//!

use super::result_response::{get_exception_result, get_success_result};
use super::utils;
use actix_web::http;
use actix_web::http::StatusCode;
use actix_web::{web, Error, HttpRequest, HttpResponse, Result};
use md5;
use std::collections::HashMap;
use wechat::{
    mp::WechatAuthorize,
    open::{get_tripartite_config, Component, Ticket, TripartiteConfig},
};
use redis::{
    get_redis_conf,
    RedisConfig
};
/// 第三方ticket推送接收处理
#[post("/wx/verify_ticket")]
pub async fn verify_ticket(
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse, Error> {
    // 获取地址栏参数
    let dic = utils::parse_query(req.query_string());
    // 获取post数据
    let post_str = utils::get_request_body(payload).await;
    log!(
        " ^^^^^ Ticket ^^^^^:  url_param: {:?} \n post_str: {:?}",
        req.query_string(),
        post_str
    );

    let tripart_config: TripartiteConfig = get_tripartite_config();
    let redis_config:RedisConfig=get_redis_conf();

    if let Err(t) = Ticket::new(tripart_config, redis_config).parse_ticket(&post_str, dic) {
        log!(" ticket parse_ticket: {:?}", t);
    };

    // 告诉服务器接收成功
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("success"))
}

/// 发起授权
#[get("/wx/auth")]
async fn auth_transfer(req: HttpRequest) -> Result<HttpResponse> {
    let query = req.query_string();
    let path = format!("/wx/official_auth?{}", query);
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(format!("<script>location.href='{}'</script>", path)))
}

/// 公众号授权
#[get("/wx/official_auth")]
async fn official_auth(req: HttpRequest) -> Result<HttpResponse> {
    let query = req.query_string();
    let dic = utils::parse_query(query);
    let mut scheme = utils::get_hq_value(&req, "x-scheme");
    if scheme.is_empty() {
        scheme = "http".to_owned();
    }

    println!(" === scheme === {:?}", scheme);
    //随机数
    let base_query = utils::get_hash_value(&dic, "q");
    // println!("base_query={:?}", base_query);
    let app_type = match base64::decode(&base_query) {
        Ok(val) => {
            let s = String::from_utf8(val).unwrap();

            let arr: Vec<&str> = s.split("|").collect();
            // println!("q={:?}", arr[3]);
            if arr.len() == 5 {
                arr[3].parse::<u32>().unwrap()
            } else {
                1
            }
        }
        Err(_) => 1,
    };
    let tripart_config: TripartiteConfig = get_tripartite_config();
    let redis_config:RedisConfig=get_redis_conf();
    let comp=Component::new(tripart_config.clone(),redis_config.clone());
  
    let result_code = match comp.create_preauthcode().await {
        Ok(code) => code,
        Err(_) => {
            match comp.create_preauthcode().await {
                Ok(code) => code,
                Err(e) => {
                    return Ok(HttpResponse::build(StatusCode::OK)
                        .content_type("text/html; charset=utf-8")
                        .body(format!("error {}", e)))
                }
            }
        }
    };
    use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
    let base_query = utf8_percent_encode(&base_query, NON_ALPHANUMERIC).to_string();
    //println!("base_query={:?}",base_query);
    let path = comp.component_login_page(
        &result_code,
        &format!(
            "{}://{}/wx/official_auth_calback?q={}",
            scheme, tripart_config.domain.clone(), base_query
        ),
        app_type,
    );
    println!("path={:?}", path);
    Ok(HttpResponse::build(StatusCode::FOUND)
        .header(http::header::LOCATION, path)
        .body(""))
}
/// 公众号授权回调
#[get("/wx/official_auth_calback")]
async fn official_auth_calback(req: HttpRequest) -> Result<HttpResponse> {
    let query = req.query_string();
    let dic = utils::parse_query(query);
    // println!("sss{:?}", req.uri().host());
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

    // println!("path={:?}", path);
    // response
    Ok(HttpResponse::build(StatusCode::FOUND)
        .header(http::header::LOCATION, path)
        .body(""))
}

/// 业务系统在完成授权以后把appid和对应的服务器机组域名回传
#[post("/wx/offical")]
async fn offical_back(_req: HttpRequest, payload: web::Payload) -> Result<HttpResponse> {
    // use crate::cluster::add_domain;
    let post_str = utils::get_request_body(payload).await;
    let dic = utils::parse_query(&post_str);
    let app_id = utils::get_hash_value(&dic, "appid");
    let domain = utils::get_hash_value(&dic, "domain");
    let authorizer_access_token = utils::get_hash_value(&dic, "authorizer_access_token");
    let authorizer_refresh_token = utils::get_hash_value(&dic, "authorizer_refresh_token");
    let is_common = match utils::get_hash_value(&dic, "is_common").parse::<bool>() {
        Ok(v) => v,
        Err(_) => false,
    };
    // add_domain(app_id.clone(), domain.clone());
    if is_common {
        use crate::official::Official;
        let mut conf = Official::new("");
        conf.appid = app_id;
        conf.authorizer_access_token = authorizer_access_token;
        conf.authorizer_refresh_token = authorizer_refresh_token;
        conf.expires_in = 7000 + utils::current_timestamp();
        conf.save("");
    }
    get_success_result("success")
}
// 业务系统在完成授权以后把appid和对应的服务器机组域名回传
#[post("/wx/common_official")]
async fn fetch_common_official(_req: HttpRequest, _payload: web::Payload) -> Result<HttpResponse> {
    use crate::official::{get_common_official, Official};
    let empty_dic: HashMap<String, String> = HashMap::new();
    let mut conf: Official = get_common_official();

    let current_expires_in = utils::current_timestamp();
    let expires_in = conf.expires_in;
    if conf.authorizer_refresh_token.is_empty() || conf.appid.is_empty() {
        conf = Official::new("");
        println!("{:?}", conf);
    }
    if conf.authorizer_refresh_token.is_empty() || conf.appid.is_empty() {
        return get_success_result(&empty_dic);
    }
    if current_expires_in > expires_in {
        let tripart_config: TripartiteConfig = get_tripartite_config();
        let redis_config:RedisConfig=get_redis_conf();
        let comp=Component::new(tripart_config.clone(),redis_config.clone());
        let auth_token: String = match comp
            .fetch_auth_token(&conf.appid, &conf.authorizer_refresh_token)
            .await
        {
            Ok(v) => v.0.clone(),
            Err(_) => "".to_owned(),
        };
        if !auth_token.is_empty() {
            conf.expires_in = utils::current_timestamp() + 7000;
            conf.authorizer_access_token = auth_token;
            conf.save("");
        } else {
            return get_success_result(&empty_dic);
        }
    }
    get_success_result(&conf)
}
#[post("/wx/test")]
async fn test(req: HttpRequest, payload: web::Payload) -> Result<HttpResponse> {
    let dic = utils::parse_query(req.query_string());
    println!("{:?}", req);
    // payload is a stream of Bytes objects
    let post_str = utils::get_request_body(payload).await;

    println!("callback {:?}, {:?}", dic, post_str);
    get_success_result("sd")
}

/// 获取第三方的token
#[post("/wx/fetch_component_token")]
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

    if md5_value != token_md5 {
        return get_exception_result("校验失败", 500);
    }
    let tripart_config: TripartiteConfig = get_tripartite_config();
    let redis_config:RedisConfig=get_redis_conf();
    let comp=Component::new(tripart_config.clone(),redis_config.clone());

    let token = comp.get_access_tokens().await;

    if token.1==0 {
        get_exception_result("获取token为空，请检查ticket是否正确推送", 500)
    } else {
        let mut content_dic: HashMap<String, String> = HashMap::new();
        content_dic.insert("token".to_owned(), token.0);
        content_dic.insert(
            "expires_in".to_owned(),
            format!("{}", token.1),
        );
        get_success_result(&content_dic)
    }
}

///获得授权url
/// x-scheme: 是在nginx的反向代理时使用, proxy_set_header X-Scheme $scheme 把请求的真实协议给定到请求头中
#[post("/wx/fetch_auth_url")]
pub async fn fetch_auth_url(req: HttpRequest, payload: web::Payload) -> Result<HttpResponse> {
    let config: TripartiteConfig = get_tripartite_config();
    // let query = req.query_string();
    let post_str = utils::get_request_body(payload).await;
    //println!("query={:?}",post_str);
    let mut scheme = utils::get_hq_value(&req, "x-scheme");
    if scheme.is_empty() {
        scheme = "http".to_owned();
    }
    let dic = utils::parse_query(&post_str);
    //随机数
    let app_id = utils::get_hash_value(&dic, "app_id");
    let domain = "".to_owned();
    // let domain = if config.wap_domain.starts_with("http") {
    //     config.wap_domain
    // } else {
    //     format!("{}://{}", scheme, config.wap_domain)
    // };

    let redirect_uri = format!("{}/wx/user_auth_calback", &domain);
    let state = utils::get_hash_value(&dic, "state");

    println!(" === redirect_uri === {:?}", redirect_uri);

    let authorize = WechatAuthorize::new(&app_id, &config.app_id, "");
    let mut scopes = Vec::new();
    scopes.push("snsapi_userinfo");
    let url = authorize.get_authorize_url(&redirect_uri, &state, &scopes, "code");
    get_success_result(&url)
}
/// 用户授权回调
#[get("/wx/user_auth_calback")]
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
                let hash_query = arr[0];
                let fkway = arr[1];
                let back_domain = arr[2].to_lowercase();
                let state = base64::encode(&format!("{}|{}|", hash_query, fkway));
                use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
                let state = utf8_percent_encode(&state, NON_ALPHANUMERIC).to_string();

                format!(
                    "{}/authback?code={}&state={}",
                    back_domain, auth_code, state
                )
            } else {
                "".to_owned()
            }
        }
        Err(_) => "".to_owned(),
    };

    // println!("path={:?}", path);
    // response
    Ok(HttpResponse::build(StatusCode::FOUND)
        // .add_default_header(http::header::LOCATION, path)
        .body(""))
}

// 微信第三方消息回调处理
// https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/Post_Application_on_the_Entire_Network/releases_instructions.html
// 上面是全网发布的资料
pub async fn callback(
    req: HttpRequest,
    path: web::Path<(String,)>,
    body: web::Bytes,
) -> Result<HttpResponse> {
    use super::wx_msg;
    let app_id = "path";
    let app_id = "&app_id";
    // 全网发布
    if app_id == "wx570bc396a51b8ff8" || app_id == "wxd101a85aa106f53e" {
        let dic = utils::parse_query(req.query_string());
        let post_str = match std::str::from_utf8(&body) {
            Ok(s) => s,
            Err(_e) => "",
        };
        log!("--- callback --- \n{:?}\n {:?}", dic, post_str);
        //wx_msg::global_publish(dic, post_str.to_owned()).await
        watch_time!(
            "global",
            wx_msg::global_publish(dic, post_str.to_owned()).await
        )
    } else {
        // 业务系统处理
        wx_msg::proxy_reply(app_id, req, body).await
    }
}
