//! copyright © ecdata.cn 2022 - present 
//! 微信服务商
//! 

#[macro_use]
extern crate serde_json;

mod apply;

// 微信服务商接口域名 
pub (crate) const API_DOMAIN: &'static str = "https://api.mch.weixin.qq.com/v3";

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
