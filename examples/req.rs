//! copyright

#[macro_use]
extern crate wechat_sdk;

fn main() {
    logs!("ee");
}
// #[macro_use]
// extern crate serde_json;

// #[tokio::main]
// async fn main() {
//     // let api_url = format!("https://api.weixin.qq.com/cgi-bin/message/custom/send?access_token={}", "self.access_token");
//     let params = json!({
//         "touser": "to_user.as_ref().to_string()",
//         "msgtype": "msg_type.as_ref().to_string()",
//         "text": {
//             "content": "content.as_ref().to_string()"
//         }
//     });
//     let res = reqwest::Client::new()
//         .post("http://127.0.0.1:8089/wx/test")
//         .json(&params)
//         .send()
//         .await
//         .unwrap();
//     println!("{:?}", res);
// }
