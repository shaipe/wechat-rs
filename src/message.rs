
use crate::WeChatResult;
use crate::wechat_crypto::WeChatCrypto;

pub struct Message  {
    token: String,
    aes_key: String,
    app_id: String,
}

impl Message {

    pub fn new(token: &str, aes_key: &str, app_id: &str) -> Self {
        Message {
            token: token.to_owned(),
            aes_key: aes_key.to_owned(),
            app_id: app_id.to_owned(),
        }
    }

    pub fn parse(
        &self,
        xml: &str,
        signature: &str,
        timestamp: i64,
        nonce: &str,
    ) -> WeChatResult<String> {
        let c = WeChatCrypto::new(&self.token, &self.aes_key, &self.app_id);
        let decrpty = c.decrypt_message(xml, signature, timestamp, nonce);
        println!("decrpty={:?}", decrpty);
        
        Ok("ticketstr".to_owned())
    }

}

