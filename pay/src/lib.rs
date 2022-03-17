//! copyright © ecdata.cn 2021 - present 
//! 微信支付模块
//! created by shaipe 20210914

mod order;
pub use order::Order;

#[macro_use]
extern crate serde_json;

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use serde_json::*;
//     use order::Order;
//     //
//     #[test]
//     fn it_pay_works() {
//         println!("{:?}","=============");
//         //初始化 配置
//         let _ = wechat_sdk::Config::load(json!({
//             "app_id": "wx455639023de66adb",
//             "mch_id": "1614479327",
//             "secret_key": "chengduhongtuikeji20210911888888",
//         }));

//         actix_rt::System::new().block_on(async {
//             let params = json!({
//                 "attach": "支付测试",
//                 "body": "testx",
//                 "nonce_str": "1212312312",
//                 "out_trade_no": "1111z",
//                 "notify_url": "https://wxpay.wxutil.com/pub_v2/pay/notify.v2",
//                 "total_fee": 1,
//                 "trade_type": "APP"
//             });
//             let r = Order::create(params).await.unwrap();
//             println!("{:?}",r);
//         });
//         //println!("{:?}",_v);
//     }
// }
