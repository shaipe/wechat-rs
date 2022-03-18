//! copyright © shaipe
//! Licensed under the Apache License

use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::{aes, blockmodes, buffer, symmetriccipher};
use std::str;
///aes 加减密
pub struct AesCrypt {
    key: Vec<u8>,
    iv: Vec<u8>,
    key_size: aes::KeySize,
}
impl AesCrypt {
    //针对key的长度，keysize动态变化
    pub fn new(_key: Vec<u8>, iv: Vec<u8>) -> AesCrypt {
        let l = _key.len();
        let mut key = _key.clone();
        let mut key_size = aes::KeySize::KeySize128;
        if l < 16 {
            key = vec_pad(_key, 16);
        } else if l > 16 && l < 24 {
            key = _key[0..16].to_vec();
        } else if l > 24 && l < 32 {
            key_size = aes::KeySize::KeySize192;
            key = _key[0..24].to_vec();
        } else if l >= 32 {
            key_size = aes::KeySize::KeySize256;
            key = _key[0..32].to_vec();
        }
        AesCrypt {
            key: key,
            iv: iv,
            key_size: key_size,
        }
    }
    /// 针对字符串进行加密
    pub fn encrypt(&self, text: String) -> String {
        //aes 加密
        let encrypted_data = aes_cbc_encrypt(self.key_size, text.as_bytes(), &self.key, &self.iv)
            .ok()
            .unwrap();
        //编码成base64
        let mut base64_encode = String::new();
        base64::encode_config_buf(&encrypted_data, base64::STANDARD, &mut base64_encode);

        base64_encode
    }
    //针对byte进行加密
    pub fn encrypt_byte(&self, text: Vec<u8>) -> String {
        //aes 加密
        let encrypted_data = aes_cbc_encrypt(self.key_size, &text, &self.key, &self.iv)
            .ok()
            .unwrap();
        //编码成base64
        let mut base64_encode = String::new();
        base64::encode_config_buf(&encrypted_data, base64::STANDARD, &mut base64_encode);

        base64_encode
    }
    /// aes解密
    /// param1: 待解密数据
    pub fn decrypt(&self, text: String) -> String {
        let mut base64_decode = Vec::<u8>::new();
        // 如是不正确的base64则返回空
        match base64::decode_config_buf(&text, base64::STANDARD, &mut base64_decode) {
            Ok(_) => {}
            Err(_e) => {
                return "".to_owned();
            }
        };
        // aes 解码
        let decrypted_data =
            match aes_cbc_decrypt(self.key_size, &base64_decode[..], &self.key, &self.iv) {
                Ok(data) => data,
                Err(_e) => {
                    println!("base64_decode={:?}", _e);
                    return "".to_owned();
                }
            };

        //转换成string
        let the_string = str::from_utf8(&decrypted_data).expect("not UTF-8");

        the_string.to_owned()
    }
}

/// Encrypt a buffer with the given key and iv using AES-256/CBC/Pkcs encryption.
fn aes_cbc_encrypt(
    key_size: aes::KeySize,
    data: &[u8],
    key: &[u8],
    iv: &[u8],
) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut encryptor = aes::cbc_encryptor(key_size, key, iv, blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = (encryptor.encrypt(&mut read_buffer, &mut write_buffer, true))?;

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
    Ok(final_result)
}

/// Decrypts a buffer with the given key and iv using AES-256/CBC/Pkcs encryption.
fn aes_cbc_decrypt(
    key_size: aes::KeySize,
    encrypted_data: &[u8],
    key: &[u8],
    iv: &[u8],
) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut decryptor = aes::cbc_decryptor(key_size, key, iv, blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = (decryptor.decrypt(&mut read_buffer, &mut write_buffer, true))?;
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

    Ok(final_result)
}
//补0
fn vec_pad(txt: Vec<u8>, length: usize) -> Vec<u8> {
    if txt.len() < length {
        let s = length - txt.len();
        let mut xs = txt;
        for i in 0..s {
            xs.push(0u8);
        }
        return xs;
    }
    txt
}
