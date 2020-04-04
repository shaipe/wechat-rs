#[macro_use]
extern crate actix_web;
#[macro_use] 
extern crate lazy_static;
extern crate wechat_sdk;

use std::{env, io};

// use actix_files as fs;
// use actix_session::{CookieSession, Session};
use actix_utils::mpsc;
use actix_web::http::{Method, StatusCode};
use actix_web::{
    error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer,
    Result,
};
use bytes::Bytes;
use bytes::{BytesMut};
use futures::StreamExt;
pub mod utils;
pub mod config;
use config::{Config,TripartiteConfig,get_tripartite_config};
use wechat_sdk::{wechat_crypto::WeChatCrypto,types::WeChatResult};
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
        Err(_e) => ""
    };

    println!("{}", post_str);

    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("nclude_str"))
}
#[post("/WxComponent.axd")]
async fn component_event( req: HttpRequest,payload: web::Payload) -> Result<HttpResponse> {
    let query = req.query_string();
    let dic=utils::parse_query(query);
    //随机数
    let nonce=utils::get_hash_value(&dic,"nonce");
    //时间缀
    let timestamp=utils::get_hash_value(&dic,"timestamp").parse::<i64>().unwrap();
    //签名信息
    let signature=utils::get_hash_value(&dic,"msg_signature");
      // payload is a stream of Bytes objects
   let post_str=get_request_body(payload).await;
   //println!("post_str={:?}",post_str);

    let tripartite:TripartiteConfig=get_tripartite_config();
    let wechat_crypto=WeChatCrypto::new(&tripartite.token,&tripartite.encoding_aes_key,&tripartite.app_id);
    let result:WeChatResult<String>=wechat_crypto.decrypt_message(&post_str, &signature, timestamp, &nonce);
    println!("{:?}",result);
  
    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("component_event"))
}
async fn get_request_body(mut payload: web::Payload)->String{
    let mut body = BytesMut::new();
    while let Some(chunk) = payload.next().await {
        // limit max size of in-memory payload
        // if (body.len() + chunk.len()) > MAX_SIZE {
        //     return Err(error::ErrorBadRequest("overflow"));
        // }
        match chunk{
            Ok(sw)=>{
                body.extend_from_slice(&sw);
            },
            Err(_)=>{

            }
        }       
    }

    let post_str = match std::str::from_utf8(&body) {
        Ok(s) => s,
        Err(_e) => ""
    };
    post_str.to_owned()
}
#[get("/auth")]
async fn index_auth( req: HttpRequest) -> Result<HttpResponse> {
    println!("{:?}", req);
    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("auth"))
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

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    Config::init("");
    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register simple route, handle all methods
            .service(index_handler)
            .service(index_auth)
            .service(component_event)
            // with path parameters
            .service(web::resource("/user/{name}").route(web::get().to(with_param)))
            // async response body
            .service(
                web::resource("/async-body/{name}").route(web::get().to(response_body)),
            )
            .service(
                web::resource("/test").to(|req: HttpRequest| match *req.method() {
                    Method::GET => HttpResponse::Ok(),
                    Method::POST => HttpResponse::MethodNotAllowed(),
                    _ => HttpResponse::NotFound(),
                }),
            )
            .service(web::resource("/error").to(|| async {
                error::InternalError::new(
                    io::Error::new(io::ErrorKind::Other, "test"),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
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
