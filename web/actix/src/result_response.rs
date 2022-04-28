//! copyright
//! 微信对接结果输出对象

use actix_web::http::StatusCode;
use actix_web::{HttpResponse, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

///返回的结构封装
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResultResponse<T> {
    #[serde(rename = "Success")]
    success: bool,
    #[serde(rename = "Code")]
    code: u32,
    #[serde(rename = "Content")]
    content: T,
    #[serde(rename = "Message")]
    message: String,
}

///成功返回
pub fn get_success_result<T:Serialize+?Sized>(content: &T) -> Result<HttpResponse> {
    let result = ResultResponse {
        success: true,
        code: 200,
        content: content.to_owned(),
        message: String::from(""),
    };
    let str_val = json!(result).to_string();
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .body(str_val))
}

///失败返回
pub fn get_exception_result(msg: &str, code: u32) -> Result<HttpResponse> {
    let result = ResultResponse {
        success: false,
        code: code,
        content: String::from(""),
        message: msg.to_owned(),
    };
    let str_val = json!(result).to_string();
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .body(str_val))
}
