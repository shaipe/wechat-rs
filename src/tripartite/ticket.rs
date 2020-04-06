use crate::types::WeChatResult;
use crate::wechat_crypto::WeChatCrypto;
use crate::xmlutil;
pub struct WechatTicket {
    token: String,
    aes_key: String,
    app_id: String,
}

impl WechatTicket {
    pub fn new(_token: &str, _aes_key: &str, _app_id: &str) -> WechatTicket {
        WechatTicket {
            token: _token.to_owned(),
            aes_key: _aes_key.to_owned(),
            app_id: _app_id.to_owned(),
        }
    }
    pub fn save_ticket(
        &self,
        xml: &str,
        signature: &str,
        timestamp: i64,
        nonce: &str,
    ) -> WeChatResult<String> {
        let c = WeChatCrypto::new(&self.token, &self.aes_key, &self.app_id);
        let decrpty = c.decrypt_message(xml, signature, timestamp, nonce);
        //println!("decrpty={:?}", decrpty);
        let ticketstr=self.parse_ticket(&decrpty.unwrap());
        Ok(ticketstr)
    }
    pub fn parse_ticket(&self, xml: &str) -> String {
        let package = xmlutil::parse(xml);
        let doc = package.as_document();
        xmlutil::evaluate(&doc, "//xml/ComponentVerifyTicket/text()").string()
    }
}
