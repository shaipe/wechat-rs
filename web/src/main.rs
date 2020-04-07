#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate lazy_static;

extern crate wechat_sdk;

use std::{env, io};

// use actix_files as fs;
// use actix_session::{CookieSession, Session};
use actix_utils::mpsc;
use actix_web::http;
use actix_web::http::{Method, StatusCode};
use actix_web::{
    error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Result,
};
use bytes::Bytes;
use bytes::BytesMut;
use futures::{
    future::{ok, Either, Ready},
    StreamExt,
};
pub mod utils;

use wechat_sdk::{
    tripartite::{
        get_tripartite_config, set_tripartite_config, TripartiteConfig, WechatComponent,
        WechatTicket,
    },
    types::WeChatResult,
};

use base64;
/// favicon handler
/// simple index handler
#[post("/")]
async fn index_handler(req: HttpRequest, mut payload: web::Payload) -> Result<HttpResponse> {
    //println!("{:?}", req);

    // payload is a stream of Bytes objects
    let mut body = BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        // if (body.len() + chunk.len()) > MAX_SIZE {
        //     return Err(error::ErrorBadRequest("overflow"));
        // }
        body.extend_from_slice(&chunk);
    }

    let post_str = match std::str::from_utf8(&body) {
        Ok(s) => s,
        Err(_e) => "",
    };

    println!("{}", post_str);

    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("nclude_str"))
}
/*
    第三方ticket
*/
#[post("/component_ticket")]
async fn component_ticket(req: HttpRequest, payload: web::Payload) -> Result<HttpResponse> {
    let query = req.query_string();
    let dic = utils::parse_query(query);
    //随机数
    let nonce = utils::get_hash_value(&dic, "nonce");
    //时间缀
    let timestamp = utils::get_hash_value(&dic, "timestamp")
        .parse::<i64>()
        .unwrap();
    //签名信息
    let signature = utils::get_hash_value(&dic, "msg_signature");
    // payload is a stream of Bytes objects
    let post_str = get_request_body(payload).await;
    //println!("post_str={:?}",post_str);

    let mut config: TripartiteConfig = get_tripartite_config();
    let t = WechatTicket::new(&config.token, &config.encoding_aes_key, &config.app_id);
    let result: WeChatResult<String> = t.save_ticket(&post_str, &signature, timestamp, &nonce);
    match result {
        Ok(val) => {
            config.access_ticket = val;
            config.save("");
            println!("config:{:?}", config);
            set_tripartite_config(config.clone());
        }
        Err(err) => {}
    };
    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("component_event"))
}
/*
    读取body里面的内容
*/
async fn get_request_body(mut payload: web::Payload) -> String {
    let mut body = BytesMut::new();
    while let Some(chunk) = payload.next().await {
        // limit max size of in-memory payload
        // if (body.len() + chunk.len()) > MAX_SIZE {
        //     return Err(error::ErrorBadRequest("overflow"));
        // }
        match chunk {
            Ok(sw) => {
                body.extend_from_slice(&sw);
            }
            Err(_) => {}
        }
    }

    let post_str = match std::str::from_utf8(&body) {
        Ok(s) => s,
        Err(_e) => "",
    };
    post_str.to_owned()
}
/*
    发起授权
*/
#[get("/auth")]
async fn official_auth(req: HttpRequest) -> Result<HttpResponse> {
    let query = req.query_string();
    let dic = utils::parse_query(query);
    //随机数
    let base_query = utils::get_hash_value(&dic, "q");

    let mut config: TripartiteConfig = get_tripartite_config();
    let token = config.get_token().await;
    //println!("access_token={:?}", token);
    let c = WechatComponent::new(&config.app_id, &config.secret, &config.access_ticket);
    let code = c.create_preauthcode(&token).await;
    //println!("code={:?}", code);
    let path = c.component_login_page(
        &code.unwrap(),
        &format!("{}/auth_calback?q={}", config.domain, base_query),
        1,
    );
    //println!("path={:?}", path);
    Ok(HttpResponse::build(StatusCode::FOUND)
        .header(http::header::LOCATION, path)
        .body(""))
}
/*
    公众号授权回调
*/
#[post("official_auth_calback")]
async fn official_auth_calback(req: HttpRequest, payload: web::Payload) -> Result<HttpResponse> {
    let query = req.query_string();
    let dic = utils::parse_query(query);
    //随机数
    let base_query = utils::get_hash_value(&dic, "q");
    let auth_code=utils::get_hash_value(&dic, "auth_code");
    let mut path="http://366kmpf.com/WebZone/Main.aspx";
    let path=match base64::decode(&base_query){
        Ok(val)=>{
            let s=String::from_utf8(val).unwrap();
          
            let arr:Vec<&str>=s.split("|").collect();
            println!("q={:?}",arr[4]);
            if arr.len()==5{
                format!("{}?p={}&auth_code={}",arr[4],base_query,auth_code)
            }
            else{
                "".to_owned()
            }
        },
        Err(_)=>{"".to_owned()}
    };

    println!("path={:?}",path);
    // response
    Ok(HttpResponse::build(StatusCode::FOUND)
        .header(http::header::LOCATION, path)
        .body(""))
}
/*
获取第三方的token
*/
#[post("fetch_component_token")]
async fn fetch_component_token(req: HttpRequest) -> Result<HttpResponse> {
    let mut config: TripartiteConfig = get_tripartite_config();
    let token = config.get_token().await;
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(token))
}
/// response body
async fn response_body(path: web::Path<String>) -> HttpResponse {
    let text = format!("Hello {}!", *path);

    let (tx, rx_body) = mpsc::channel();
    let _ = tx.send(Ok::<_, Error>(Bytes::from(text)));

    HttpResponse::Ok().streaming(rx_body)
}

/// handler with path parameters like `/user/{name}/`
async fn with_param(req: HttpRequest, path: web::Path<(String,)>) -> HttpResponse {
    println!("{:?}", req);

    HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Hello {}!", path.0))
}
/*
获取第三方的token
*/
#[get("index")]
async fn index(req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("<a href='http://b2b323.366ec.net/auth?q=M3wxfDJ8MXxodHRwOi8vd3d3LjM2NmttcGYuY29tL1dlYlpvbmUvU29jaWFsL1dlY2hhdFNldC5hc3B4'>222</a>"))
}
#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    TripartiteConfig::init("");
    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            .service(index)
            // register simple route, handle all methods
            .service(index_handler)
            .service(component_ticket)
            .service(official_auth)
            .service(official_auth_calback)
            .service(fetch_component_token)
            // with path parameters
            .service(web::resource("/user/{name}").route(web::get().to(with_param)))
            // async response body
            .service(web::resource("/async-body/{name}").route(web::get().to(response_body)))
            .service(
                web::resource("/test").to(|req: HttpRequest| match *req.method() {
                    Method::GET => HttpResponse::Ok(),
                    Method::POST => HttpResponse::MethodNotAllowed(),
                    _ => HttpResponse::NotFound(),
                }),
            )
            .service(web::resource("/error").to(|| {
                async {
                    error::InternalError::new(
                        io::Error::new(io::ErrorKind::Other, "test"),
                        StatusCode::INTERNAL_SERVER_ERROR,
                    )
                }
            }))
        // // default
        // .default_service(
        //     // 404 for GET request
        //     web::resource("")
        //         .route(web::get().to(p404))
        //         // all requests that are not `GET`
        //         .route(
        //             web::route()
        //                 .guard(guard::Not(guard::Get()))
        //                 .to(HttpResponse::MethodNotAllowed),
        //         ),
        // )
    })
    .bind("127.0.0.1:8089")?
    .run()
    .await
}
