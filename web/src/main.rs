//! copyright © ecdata.cn 2021 - present
//! 应用
//! created by shaipe 20210303

#[macro_use]
extern crate actix_web;

// #[macro_use]
// extern crate wechat;
#[macro_use]
extern crate wechat_sdk;
extern crate redis;
use wechat_sdk::WechatError;

#[macro_use]
extern crate lazy_static;

// use awc::Client;

use actix_web::http::StatusCode;
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result};
// mod cluster;
mod official;
mod result_response;
mod utils;
mod wx_handler;
mod wx_msg;
#[get("/")]
async fn index_handler(_req: HttpRequest) -> Result<HttpResponse> {
    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("wx test"))
}

/// 应用启动入口
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 加载配置文件
    let conf_path = "conf/social.yml";

    // 启动web服务
    start_web_server(conf_path).await
}

/// web服务启动
async fn start_web_server(_conf_path: &str) -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    // // 加载配置信息
    // let conf = match Config::load_yaml(conf_path) {
    //     Ok(conf) => conf,
    //     Err(e) => {
    //         println!("file: {}, {:?}", conf_path, e);
    //         Config::default()
    //     }
    // };

    // println!("config :: {:?}", conf);

    // 设置服务器运行ip和端口信息
    // let ip = format!("{}:{}", conf.web.ip, conf.web.port);
    let ip = format!("{}:{}", "0.0.0.0", 999);

    // 启动一个web服务
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            // .wrap(
            //     // 设置允许跨域请求
            //     actix_cors::Cors::default()
            //         .allow_any_origin()
            //         .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            //         .max_age(3600),
            // )
            // .app_data(Client::new())
            .service(index_handler)
            .service(wx_handler::verify_ticket)
            .service(web::resource("/wx/cback/{appid}").route(web::post().to(wx_handler::callback)))
    })
    .bind(ip)?
    .run()
    .await
}
