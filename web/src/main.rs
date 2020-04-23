//! copyright
//! 微信业务处理服务入口类

#[macro_use]
extern crate actix_web;

#[macro_use]
extern crate wechat_sdk;

#[macro_use]
extern crate lazy_static;

use std::{env, io};
use actix_web::http::{StatusCode};
use actix_web::{
    middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result,
};

mod utils;
mod cluster;
mod wx_msg;
mod wx_handler;
mod config;
mod result_response;
use cluster::load_cluster;

use wechat_sdk::tripartite::Ticket;

#[get("/")]
async fn index_handler(_req: HttpRequest) -> Result<HttpResponse> {
    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("wx test"))
}



#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    // 加载应用id与域名的映射信息
    load_cluster("");
    
    let conf = config::Config::new("");
    Ticket::new("");
    let addr = format!("0.0.0.0:{}", conf.port);
    
    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // .data(Client::new())
            // register simple route, handle all methods
            .service(index_handler)
            .service(wx_handler::receive_ticket)
            .service(wx_handler::auth_transfer)
            .service(wx_handler::official_auth)
            .service(wx_handler::official_auth_calback)
            .service(wx_handler::offical_back)
            .service(wx_handler::fetch_component_token)
            .service(wx_handler::fetch_auth_url)
            .service(wx_handler::user_auth_calback)
            .service(wx_handler::test)
            // with path parameters
            .service(web::resource("/wx/cback/{appid}").route(web::post().to(wx_handler::callback)))
            
    })
    .bind(addr)?
    .run()
    .await
}
