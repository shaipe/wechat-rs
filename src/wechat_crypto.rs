//! copyright
//! 微信加解密码处理

use crate::errors::WeChatError;
use crate::WeChatResult;
use byteorder::{NativeEndian, WriteBytesExt, ReadBytesExt};
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use crypto::{aes, blockmodes, buffer, symmetriccipher};
use std::collections::HashMap;
use std::io::Cursor;

use rand::thread_rng;
use rand::Rng;

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
        // println!("{:?}",encoding_aes_key);
        // let c= Config::new(CharacterSet::Crypt,true);
        // c.decode_allow_trailing_bits(true);

        // let key = base64::decode_config(&aes_key,c).unwrap();
        // println!("{:?}",key);
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
    ) -> WeChatResult<String> {
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
            return Err(WeChatError::InvalidSignature);
        }
        let msg = self.decrypt(&encrypted_msg)?;
        Ok(msg)
    }

    /// 解密
    pub fn decrypt(&self, ciphertext: &str) -> WeChatResult<String> {
        let b64decoded = base64::decode(ciphertext).unwrap();
        // aes descrypt
        let text = aes256_cbc_decrypt(&b64decoded, &self.key, &self.key[..16]).unwrap();

        let mut rdr = Cursor::new(text[16..20].to_vec());
        let content_length = u32::from_be(rdr.read_u32::<NativeEndian>().unwrap()) as usize;
        let content = &text[20..content_length + 20];
        let from_id = &text[content_length + 20..];
        // println!("form_id: {:?}  ,, id_ {:?}", from_id, self._id.as_bytes());
        // 此处取出的formid中包含了回车符,只能取前18位进行判断比较
        if &from_id[0..18] != self._id.as_bytes() {
            return Err(WeChatError::InvalidAppId);
        }
        let content_string = String::from_utf8(content.to_vec()).unwrap();
        Ok(content_string)
    }
    //随机数
    fn get_random_string(&self) -> String {
        if cfg!(test) {
            "1234567890123456".to_owned()
        } else {
           thread_rng().gen_ascii_chars().take(16).collect()
        }
    }
    pub fn encrypt_message(&self, msg: &str, timestamp: i64, nonce: &str) -> WeChatResult<String> {
        let mut wtr = self.get_random_string().into_bytes();
        //采用低位编址
        wtr.write_u32::<NativeEndian>((msg.len() as u32).to_be()).unwrap();
        wtr.extend(msg.bytes());
        wtr.extend(self._id.bytes());
        //aes 加密
        let encrypted = aes256_cbc_encrypt(&wtr, &self.key, &self.key[..16]).unwrap();
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
            encrypt=b64encoded,
            signature=signature,
            timestamp=timestamp,
            nonce=nonce,
        );
        Ok(msg)
    }
}

/// 从HashMap中取值
fn get_hash_value(query_params: &HashMap<String, String>, key: &str) -> String {
    match query_params.get(key) {
        Some(val) => val.clone(),
        None => "".to_owned(),
    }
}

/// Decrypts a buffer with the given key and iv using AES-256/CBC/Pkcs encryption.
pub fn aes256_cbc_decrypt(
    encrypted_data: &[u8],
    key: &[u8],
    iv: &[u8],
) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    // 此处的最后一个参数要使用不直充的方式才行
    let mut decryptor =
        aes::cbc_decryptor(aes::KeySize::KeySize256, key, iv, blockmodes::NoPadding);
    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);
    loop {
        match decryptor.decrypt(&mut read_buffer, &mut write_buffer, true) {
            Ok(result) => {
                final_result.extend(
                    write_buffer
                        .take_read_buffer()
                        .take_remaining()
                        .iter()
                        .map(|&i| i),
                );
                match result {
                    BufferResult::BufferUnderflow => break,
                    BufferResult::BufferOverflow => {}
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
    Ok(final_result)
}
// Encrypt a buffer with the given key and iv using AES-256/CBC/Pkcs encryption.
fn aes256_cbc_encrypt(data: &[u8],key: &[u8], iv: &[u8])->Result<Vec<u8>,symmetriccipher::SymmetricCipherError>{
    let mut encryptor=aes::cbc_encryptor(
        aes::KeySize::KeySize256,
        key,
        iv,
        blockmodes::PkcsPadding);

    let mut final_result=Vec::<u8>::new();
    let mut read_buffer=buffer::RefReadBuffer::new(data);
    let mut buffer=[0;4096];
    let mut write_buffer=buffer::RefWriteBuffer::new(&mut buffer);
    loop{
        let result=(encryptor.encrypt(&mut read_buffer,&mut write_buffer,true))?;

        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

        match result {
            BufferResult::BufferUnderflow=>break,
            BufferResult::BufferOverflow=>{},
        }
    }

    Ok(final_result)
}

#[cfg(test)]
mod tests {
    use super::WeChatCrypto;

    #[test]
    fn test_decrypt_message() {
        let xml = "<xml>
        <Encrypt><![CDATA[zx9TFkPBdRzgd5WzEk69P/kXUTgTagYs8Q1cUc2Hn2GMD2mJj7tqKA46gxAcexRbwXbsBMk9nBi86Hp/IhjgxnH60ppOT5apysAB2dMnT40AtEVHcP26XWhTDa0YCHYS08as9r3A+pCmAf4bzsFvVZG2VJoejZxH0S0tgMExAt5BOtFc8T7dNLAYNEUBfC+YW/tHMqTp2EaJdbp8I1sSDzvbbP0+Le+TjGXt57Fl2AXwfeNgVi3H+sbT39uNXQscvSKw2zl5AZ9V/6VSkPydCgvFZuM6nujZDEMeqnRckPibuLavfZlX210ebSYWNu+h6rz0G5IKxnmO4bSxAGjMB6yBASKwNk+Ne/lcxdrZNoWtGo7KhCMUWBM39P+3cLlOiEguXD9Z+DXfLjiQBauwk6b0J6fEPF/qxXsx4VqURugCRM8NvgsxDLFbpIuG+MB0SQ1CpLp1m/nXqWf8bVMGF23yoIW1VxvXf56b09S12MM=]]></Encrypt>
        <MsgSignature><![CDATA[7c6b47b57a7608b2d37fd15fa2acd31e14946909]]></MsgSignature>
        <TimeStamp>1586949687</TimeStamp>
        <Nonce><![CDATA[1586949687]]></Nonce>
        </xml>";
        let crypto = WeChatCrypto::new("shaipe", "kdjCGGJKSRjjhESfPO5lTSWtYS0v5pQX47skCkZczio", "wxce775970ff046a47");
        use std::collections::HashMap;
        let mut dic=HashMap::new();
        dic.insert("msg_signature".to_owned(),"7c6b47b57a7608b2d37fd15fa2acd31e14946909".to_owned());
        dic.insert("nonce".to_owned(),"1586949687".to_owned());
        dic.insert("timestamp".to_owned(),"1586949687".to_owned());
        let decrypted = crypto
            .decrypt_message(xml, &dic)
            .unwrap();
        println!("decrypted={:?}", decrypted);
    }

    #[test]
    fn test_encrypt_message(){
        let msg=r#"<xml>
        <ToUserName><![CDATA[ozy4qt5QUADNXORxCVipKMV9dss0]]></ToUserName>
        <FromUserName><![CDATA[gh_3c884a361561]]></FromUserName>
        <CreateTime>1586937584</CreateTime>
        <MsgType><![CDATA[text]]></MsgType>
        <Content><![CDATA[TESTCOMPONENT_MSG_TYPE_TEXT_callback]]></Content>
        </xml>"#;
        let crypto = WeChatCrypto::new("shaipe", "kdjCGGJKSRjjhESfPO5lTSWtYS0v5pQX47skCkZczio", "wxce775970ff046a47");
        let timestamp = crate::current_timestamp();
        let nonce = format!("{}", timestamp);
        let encrypt_text = crypto.encrypt_message(msg, timestamp, &nonce);
        println!("{:?}",encrypt_text);
    }
}
