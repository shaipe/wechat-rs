use crate::wechat_crypto::WeChatCrypto;
use crate::WeChatResult;
use std::collections::HashMap;


// pub struct Message {
//     to_user_name
// }



// pub struct Message {
//     token: String,
//     aes_key: String,
//     app_id: String,
// }

// impl Message {
//     pub fn new(token: &str, aes_key: &str, app_id: &str) -> Self {
//         Message {
//             token: token.to_owned(),
//             aes_key: aes_key.to_owned(),
//             app_id: app_id.to_owned(),
//         }
//     }

//     pub fn parse(&self, xml: &str, query_params: HashMap<String, String>) -> WeChatResult<String> {
        
//         let c = WeChatCrypto::new(&self.token, &self.aes_key, &self.app_id);
//         let decrpty = c.decrypt_message(xml, query_params);
//         println!("decrpty={:?}", decrpty);
//         Ok("ticketstr".to_owned())
//     }
// }


