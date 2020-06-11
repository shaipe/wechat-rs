use actix_web::{web, HttpRequest};
use std::collections::HashMap;

/// 获取请求对象头的值
#[allow(dead_code)]
pub fn get_header_value_str(req: &HttpRequest, key: &str) -> String {
    let some_val = req.head().headers.get(key);
    match some_val {
        Some(t) => t.to_str().unwrap().to_string(),
        None => "".to_owned(),
    }
}

/// 解析地址栏参数
pub fn parse_query(query_string: &str) -> HashMap<String, String> {
    if query_string.is_empty() {
        return HashMap::new();
    }
    let q_a: Vec<&str> = query_string.split("&").collect();
    let mut res: HashMap<String, String> = HashMap::new();
    use percent_encoding::percent_decode;
    for s in q_a {
        // let ss: &str = s;
        let kv: Vec<&str> = s.split("=").collect();
        let kvalue = percent_decode(kv[1].as_bytes())
        .decode_utf8()
        .unwrap();
        res.insert(kv[0].to_string(), kvalue.to_string());
    }
    res
}
/// 获取头部地址栏参数据的值
/// param1: httpRequest对象
/// param2: 获取关键字
#[allow(dead_code)]
pub fn get_hq_value(req: &HttpRequest, key: &str) -> String {
    let mut val = get_header_value_str(&req, key);
    if val.is_empty() {
        let q_str: &str = req.query_string();
        let query_params: HashMap<String, String> = parse_query(q_str);
        val = match query_params.get(key) {
            Some(val) => val.clone(),
            None => "".to_owned(),
        };
    }
    val
}

/// 获取访问者的ip
#[allow(dead_code)]
pub fn get_ip(req: &HttpRequest) -> String {
    use std::net::IpAddr::V4;
    // use std::net::SocketAddr::V4;
    let addr = req.peer_addr();
    if let Some(ad) = addr {
        if let V4(ip) = ad.ip() {
            return format!("{:?}", ip);
        }
    }
    "127.0.0.1".to_owned()
}

/// 从Hashmap中取值
pub fn get_hash_value(query_params: &HashMap<String, String>, key: &str) -> String {
    match query_params.get(key) {
        Some(val) => val.clone(),
        None => "".to_owned(),
    }
}

/// 读取body里面的内容
pub async fn get_request_body(mut payload: web::Payload) -> String {
    use bytes::BytesMut;
    use futures::StreamExt;
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
pub fn current_timestamp() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}