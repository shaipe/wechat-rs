use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use actix_web::http::{StatusCode};
use actix_web::{HttpResponse,Result};
///返回的结构封装
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResultResponse<T>{
    Success:bool,
    Code:u32,
    Content:T,
    Message:String
}
///成功返回
pub fn get_success_result(_content:&HashMap<String,String>) -> Result<HttpResponse>{
    let result=ResultResponse{
        Success:true,
        Code:200,
        Content:_content.to_owned(),
        Message:String::from(""),
    };
    let str_val = json!(result).to_string();
    Ok(HttpResponse::build(StatusCode::OK)
    .content_type("application/json; charset=utf-8")
    .body(str_val))
}
pub fn get_success_result2(_content:&str) -> Result<HttpResponse>{
    let result=ResultResponse{
        Success:true,
        Code:200,
        Content:_content.to_owned(),
        Message:String::from(""),
    };
    let str_val = json!(result).to_string();
    Ok(HttpResponse::build(StatusCode::OK)
    .content_type("application/json; charset=utf-8")
    .body(str_val))
}
///失败返回
pub fn get_exception_result(msg:&str,code:u32) -> Result<HttpResponse>{
    let result=ResultResponse{
        Success:false,
        Code:code,
        Content:String::from(""),
        Message:msg.to_owned()
    };
    let str_val = json!(result).to_string();
    Ok(HttpResponse::build(StatusCode::OK)
    .content_type("application/json; charset=utf-8")
    .body(str_val))
}