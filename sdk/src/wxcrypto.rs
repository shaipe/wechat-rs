//! copyright
//! 微信加解密码处理

use crate::WechatResult as Result;
use byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use crypto::{aes, blockmodes, buffer};
use std::collections::HashMap;
use std::io::Cursor;
use crate:: AesCrypt;
// use rand::thread_rng;
// use rand::Rng;

#[derive(Debug, Eq, PartialEq)]
pub struct WeChatCrypto {
    token: String,
    key: Vec<u8>,
    _id: String,
}

impl WeChatCrypto {
    /// new
    pub fn new(token: &str, encoding_aes_key: &str, _id: &str) -> WeChatCrypto {
        let mut aes_key = encoding_aes_key.to_owned();
        aes_key.push('=');
        let key = base64::decode(&aes_key).unwrap();
        WeChatCrypto {
            token: token.to_owned(),
            key: key,
            _id: _id.to_owned(),
        }
    }

    /// 获取签名
    fn get_signature(&self, timestamp: i64, nonce: &str, encrypted: &str) -> String {
        let mut data = vec![
            self.token.clone(),
            timestamp.to_string(),
            nonce.to_owned(),
            encrypted.to_owned(),
        ];
        data.sort();
        let data_str = data.join("");

        // sha1
        let mut hasher = Sha1::new();

        // write input message
        hasher.input_str(&data_str);

        // read hash digest
        hasher.result_str()
    }

    /// 消息解密
    pub fn decrypt_message(
        &self,
        xml: &str,
        query_params: &HashMap<String, String>,
    ) -> Result<String> {
        //随机数
        let nonce = get_hash_value(query_params, "nonce");
        //时间缀
        let timestamp = match get_hash_value(query_params, "timestamp").parse::<i64>() {
            Ok(v) => v,
            Err(_e) => 0,
        };
        //签名信息
        let signature = get_hash_value(query_params, "msg_signature");

        use super::xmlutil;
        let package = xmlutil::parse(xml);
        let doc = package.as_document();
        let encrypted_msg = xmlutil::evaluate(&doc, "//xml/Encrypt/text()").string();
        // println!("encrypted_msg={:?}",encrypted_msg);
        let real_signature = self.get_signature(timestamp, &nonce, &encrypted_msg);

        // println!("o: {}, new: {}", signature, real_signature);

        if signature != real_signature {
            return Err(error! {
                code: 40002,
                msg: "Invalid Signature",
            });
        }
        let msg = self.decrypt(&encrypted_msg)?;
        log!("######### decode message ########## \n{}", msg);
        Ok(msg)
    }

    /// 解密
    pub fn decrypt(&self, ciphertext: &str) -> Result<String> {
        
        let aes=AesCrypt::new(self.key.clone(),self.key[..16].to_vec());
        let content=aes.decrypt(ciphertext.to_owned());
        // aes descrypt
        let text =content.as_bytes();

        let mut rdr = Cursor::new(text[16..20].to_vec());
        let content_length = u32::from_be(rdr.read_u32::<NativeEndian>().unwrap()) as usize;
        let content = &text[20..content_length + 20];
        let from_id = &text[content_length + 20..];
        //println!("form_id: {:?}  ,, id_ {:?}", from_id, self._id.as_bytes());
        // 此处取出的formid中包含了回车符,只能取前18位进行判断比较
        if &from_id[0..18] != self._id.as_bytes() {
            return Err(error! {code: 50001, msg: "Invalid from"});
        }
        let content_string = String::from_utf8(content.to_vec()).unwrap();
        Ok(content_string)
    }
    /// 对消息进行加密
    pub fn encrypt_message(&self, msg: &str, timestamp: i64, nonce: &str) -> Result<String> {
        let rnd_str = get_random_string(16);
        let mut wtr = rnd_str.into_bytes();

        // log!(format!("%%%%%%%%%%%%%%%%%%% rnd str %%%%%%%%%%%%%%%%%%%%%%%%% \n{}  --- {:?}", rnd_str, wtr));

        //采用低位编址
        wtr.write_u32::<NativeEndian>((msg.len() as u32).to_be())
            .unwrap();
        wtr.extend(msg.bytes());
        wtr.extend(self._id.bytes());
        //aes 加密
        let aes=AesCrypt::new(self.key.clone(),self.key[..16].to_vec());
        let encrypted =aes.encrypt_byte(wtr); //aes256_cbc_encrypt(&wtr, &self.key, &self.key[..16]).unwrap();
        //base64 编码
        let b64encoded = base64::encode(&encrypted);
        //获得签名
        let signature = self.get_signature(timestamp, nonce, &b64encoded);
        let msg = format!(
            "<xml>\n\
            <Encrypt><![CDATA[{encrypt}]]></Encrypt>\n\
            <MsgSignature><![CDATA[{signature}]]></MsgSignature>\n\
            <TimeStamp>{timestamp}</TimeStamp>\n\
            <Nonce><![CDATA[{nonce}]]></Nonce>\n\
            </xml>",
            encrypt = b64encoded,
            signature = signature,
            timestamp = timestamp,
            nonce = nonce,
        );

        log!(
            "#################### encode message #####################\n{}",
            msg
        );
        Ok(msg)
    }
}

/// 获取随机字符串
fn get_random_string(length: usize) -> String {
    use rand::prelude::*;
    // use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789"; // )(*&^%$#@!~
    let mut rng = rand::thread_rng();
    let mut nums: Vec<i32> = (1..CHARSET.len() as i32).collect();
    nums.shuffle(&mut rng);

    let y = &nums[0..length];
    let rnd_string: String = y
        .iter()
        .map(|&x| {
            let idx = x as usize;
            CHARSET[idx] as char
        })
        .collect();

    rnd_string
}

/// 从HashMap中取值
fn get_hash_value(query_params: &HashMap<String, String>, key: &str) -> String {
    match query_params.get(key) {
        Some(val) => val.clone(),
        None => "".to_owned(),
    }
}


#[cfg(test)]
mod tests {
    use super::WeChatCrypto;

    #[test]
    fn test_decrypt_message() {
        let xml = "<xml>
        <Encrypt><![CDATA[vtVuL2vCakdSsoeEIieqx4CBzJLNdZQd3YishL1Qx70/C3pSKJdLVZcBkwpYZM4FvAh38eGo7CJ5GPSm9/sLU4O+zP9HRz5k1ltvw5fHUdmBDSAVZHvxRHTMxgXVoTr04JAMoSw3InJWrkU5hQIzO8mtgs3ypOOE1cNCSwblwqh690mHi+XExtDRL+OC8DJnhuH2b46k7BpwPWh9z1OP2INoQMYsa6HLfrOfobvRI3HQwjMKCrLG3dfK0j7nWbqLHsmKlaoTcrYfqzvuBCVkbvP+KOr2xo97c+JdKVBafrWc5h4VY2oM6xk83imVkkg7yQGfqvEua3milUeo0aX0sIcz2PZCVK8qc1NG/cFYz9SiQGBMPm0Hvf5fLSAgP2EzQVZNapWBk4cVZkzNFHWuODz0g8Z2mwKBrqF9oIFJX55uoRqAHQf39OLvl0VTVUMx0kzEHN3F8Qydz9d5Kh3/JJz6rwbRYpJZOMxWPVJecw4MWfG/iE4XG7u7AzEV7bbeBgg53JASoVCj2Nd7j70sUw==]]></Encrypt>
        <MsgSignature><![CDATA[b135f073fee09b86d9a0b83fcbea58a0e6569299]]></MsgSignature>
        <TimeStamp>1587121403</TimeStamp>
        <Nonce><![CDATA[1661748508]]></Nonce>
        </xml>";
        let crypto = WeChatCrypto::new(
            "tokenkm323",
            "khEda6IkyCedf2pbl7kKGX2N42bhOWLbkyBDgkkmpfs",
            "wx618efe0d63406d44",
        );
        use std::collections::HashMap;
        let mut dic = HashMap::new();
        dic.insert(
            "msg_signature".to_owned(),
            "b135f073fee09b86d9a0b83fcbea58a0e6569299".to_owned(),
        );
        dic.insert("nonce".to_owned(), "1661748508".to_owned());
        dic.insert("timestamp".to_owned(), "1587121403".to_owned());
        let decrypted = crypto.decrypt_message(xml, &dic).unwrap();
        println!("decrypted={:?}", decrypted);
    }

    #[test]
    fn test_encrypt_message() {
        let msg = r#"<xml><ToUserName><![CDATA[gh_3c884a361561]]></ToUserName>
        <FromUserName><![CDATA[ozy4qt5QUADNXORxCVipKMV9dss0]]></FromUserName>
        <CreateTime>1587087558</CreateTime>
        <MsgType><![CDATA[text]]></MsgType>
        <Content><![CDATA[QUERY_AUTH_CODE:queryauthcode@@@S46RZzudLRYEjxbNd5rzokMIybrsHw8a-Bm1gNX1PWyx_PFOhyilIHnnT6PKgdTkSOFAkQgosaogCOB-ZV62vg]]></Content>
        <MsgId>6816489157906572969</MsgId>
        </xml>"#;
        let crypto = WeChatCrypto::new(
            "shaipe",
            "kdjCGGJKSRjjhESfPO5lTSWtYS0v5pQX47skCkZczio",
            "wxce775970ff046a47",
        );
        let timestamp = crate::current_timestamp();
        let nonce = format!("{}", timestamp);
        let encrypt_text = crypto.encrypt_message(msg, timestamp, &nonce);
        println!("{:?}", encrypt_text);
    }
}
