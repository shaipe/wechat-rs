//! copyright © ecdata.cn 2021 - present
use wechat_sdk::{aes128_cbc_decrypt, Client, WechatResult};


pub struct Order;


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
    pub fn create() -> WechatResult<serde_json::Value> {
        let api_url ="https://api.mch.weixin.qq.com/v3/pay/transactions/jsapi";

        let data = json!({
            "amount": { 
                "total": 0.01,
                "currency": "CNY"
            },
            "mchid": "",
            "description": "",
            "notify_url": "",
            "payer": {
                "openid": "",
                "out_trade_no": "",
                "goods_tag": "",
                "appid": ""
            }
        });

        Ok(data)
        
    }
}