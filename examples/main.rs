#[macro_use]
extern crate actix_web;

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

/// favicon handler
/// simple index handler
#[post("/")]
async fn index_handler(req: HttpRequest, mut payload: web::Payload) -> Result<HttpResponse> {
    println!("{:?}", req);

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

    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register simple route, handle all methods
            .service(index_handler)
            .service(index_auth)
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
