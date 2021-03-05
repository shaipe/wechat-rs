//! copyright © ecdata.cn 2021 - present
//! 微信公众号授权


const WECHAT_OPEN_URI: &'static str = "https://open.weixin.qq.com";
///网页授权
pub struct WechatAuthorize {
    app_id: String,
    com_app_id: String,
    // com_access_token: String
}
impl WechatAuthorize {
    ///
    pub fn new(_app_id: &str, _com_app_id: &str, _com_access_token: &str) -> WechatAuthorize {
        WechatAuthorize {
            app_id: _app_id.to_string(),
            com_app_id: _com_app_id.to_string(),
            // com_access_token: _com_access_token.to_string(),
        }
    }
    /// 授权页面
    pub fn get_authorize_url(
        &self,
        redirect_uri: &str,
        state: &str,
        scope: &Vec<&str>,
        response_type: &str,
    ) -> String {
        use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

        let encode_uri = if redirect_uri.starts_with("http") {
            utf8_percent_encode(redirect_uri, NON_ALPHANUMERIC).to_string()
        } else {
            utf8_percent_encode(&format!("http://{}", redirect_uri), NON_ALPHANUMERIC).to_string()
        };

        let uri=format!("{}{}",WECHAT_OPEN_URI,format!("/connect/oauth2/authorize?appid={}&redirect_uri={}&response_type={}&scope={}&state={}&component_appid={}#wechat_redirect",
        self.app_id,encode_uri,response_type,scope.join(","),state,self.com_app_id));

        println!("authorize url: {}", uri);
        
        uri
    }
}
