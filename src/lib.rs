//! copyright © shaipe 2020 - persent
//!

// #[allow(unused_imports)]
// #[macro_use]
// extern crate wechat_sdk;
//use wechat_sdk::WechatError;

pub mod open {
    #[cfg(feature = "open")]
    pub use wechat_open::*;
}
/// 微信公众号
pub mod mp {
    #[cfg(feature = "mp")]
    pub use wechat_mp::*;
}

/// 微信小程序
pub mod weapp {
    #[cfg(feature = "weapp")]
    pub use wechat_weapp::*;
}

/// 微信支付
pub mod pay {
    
    pub use wechat_pay::*;
}

/// 微信小店
pub mod store {
    #[cfg(feature = "store")]
    pub use wechat_store::*;
}

/// 企业微信
pub mod work {
    #[cfg(feature = "work")]
    pub use wechat_work::*;
}


pub use wechat_sdk::*;


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::*;
    use wechat_pay::Order;
    //
    #[test]
    fn it_pay_ok_works() {
        println!("{:?}","=======================");
        //初始化 配置
        let conf = wechat_sdk::Config::load(json!({
            "app_id": "",
            "mch_id": "",
            "secret_key": "",
            "private_key": "cert/apiclient_key.pem",
            "certificate": "cert/apiclient_cert.pem"
        })).unwrap();

        actix_rt::System::new().block_on(async {
            let _params = json!({
                "attach": "支付测试",
                "body": "testx",
                "nonce_str": "1212312312",
                "out_trade_no": "11112z",
                "notify_url": "https://wxpay.wxutil.com/pub_v2/pay/notify.v2.php",
                "total_fee": 1,
                "openid": "oUpe_5T08-CTL7sXr_XwchZmlQu4",
                "trade_type": "JSAPI"
            });

            let params = json!({
                "nonce_str": "1212312312",
                "out_trade_no": "A_4425320788525061",
                "out_refund_no": "saa1234567891",
                "total_fee": 1,
                "refund_fee": 1,
                //"notify_url": "https://wxpay.wxutil.com/pub_v2/pay/notify.v2.php",
            });
            let r = Order::new(conf);
            let v = r.refund(params).await;
            println!("{:?}",v);
            //let r = Order::create(params).await.unwrap();
            //println!("{:?}",r);
        });
        //println!("{:?}",_v);
    }
}