//! copyright © ecdata.cn 2021 - present
use wechat_sdk::{Client, WechatResult};
use itertools::Itertools;
use wechat_sdk::Config;
use chrono::prelude::{ Utc };
pub struct Order{

}


/// // 请求body参数
// String reqdata = "{"
// + "\"amount\": {"
// + "\"total\": 100,"
// + "\"currency\": \"CNY\""
// + "},"
// + "\"mchid\": \"1900006891\","
// + "\"description\": \"Image形象店-深圳腾大-QQ公仔\","
// + "\"notify_url\": \"https://www.weixin.qq.com/wxpay/pay.php\","
// + "\"payer\": {"
// + "\"openid\": \"o4GgauE1lgaPsLabrYvqhVg7O8yA\"" + "},"
// + "\"out_trade_no\": \"1217752501201407033233388881\","
// + "\"goods_tag\": \"WXG\","
// + "\"appid\": \"wxdace645e0bc2c424\"" + "}"; 

impl Order {

    // 统一下单
    pub async fn create(mut params: serde_json::Value) -> WechatResult<serde_json::Value> {
        let api_url = "https://api.mch.weixin.qq.com/pay/unifiedorder";
        let conf = Config::get();
       
        
        params["appid"] = serde_json::Value::String(conf.app_id);
        params["mch_id"] = serde_json::Value::String(conf.mch_id);
        //api 
        let secret_key = conf.secret_key;
      
        let mut vs = vec![];
        if let Some(map) = params.as_object() {
            for key in map.keys().sorted() {
                vs.push(format!("{}={}", key,map[key].to_string().trim_matches(&['"','"'] as &[_])));
            }
            vs.push(format!("{}={}", "key",secret_key));    
            let wait_md5_str = format!("{}",vs.join("&"));
            let sign = format!("{:x}",md5::compute(&wait_md5_str)).to_uppercase();
            params["sign"] = serde_json::Value::String(sign);
        }
        
        
        let body = format!("<xml>{}</xml>",serde_xml_rs::to_string(&params).unwrap_or_default());
        
        let request = Client::new();
        let r = request.post(&api_url,&body).await.unwrap_or_default();
        
        let ps = serde_xml_rs::from_str::<serde_json::Value>(&r).unwrap_or_default();
         // 交易类型
         let trade_type = params["trade_type"].as_str();
         
        if let Some(root_doms) = ps.as_object() {
            let rr: serde_json::Map<String,serde_json::Value> = root_doms.iter().map(|(i,vo)| (i.clone(),vo["$value"].clone())).collect();
            
            let prepayid = rr.get("prepay_id");
           
            if trade_type == Some("APP") && prepayid.is_some() { //二次签名
                let mut new_params = serde_json::Map::new();
                let mut vs = vec![];
                let timestamp = Utc::now().naive_local().timestamp();
                new_params.insert(format!("appid"), rr.get("appid").unwrap().clone());
                new_params.insert(format!("partnerid"), rr.get("mch_id").unwrap().clone());
                new_params.insert(format!("prepayid"), rr.get("prepay_id").unwrap().clone());
                new_params.insert(format!("package"), json!("Sign=WXPay"));
                new_params.insert(format!("noncestr"), rr.get("nonce_str").unwrap().clone());
                new_params.insert(format!("timestamp"), json!(timestamp));
                
                for key in new_params.keys().sorted() {
                    vs.push(format!("{}={}", key,new_params[key].to_string().trim_matches(&['"','"'] as &[_])));
                }
                vs.push(format!("{}={}", "key",secret_key));    
                let wait_md5_str = format!("{}",vs.join("&"));
                let sign = format!("{:x}",md5::compute(&wait_md5_str)).to_uppercase();
                new_params.insert(format!("sign"), json!(sign));
                return Ok(serde_json::Value::Object(new_params));
            }

            return Ok(serde_json::Value::Object(rr));
        }

        Ok(json!({
            "return_code": "FAIL",
            "return_msg": "请求网络问题"
        }))
        
    }
}