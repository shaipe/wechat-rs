//! copyright © ecdata.cn 2021 - present
//! 小程序订阅模版

use wechat_sdk::{Client, WechatResult};

use serde::{Deserialize, Serialize};

/// 订阅模版关键词
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SubTemplateKeyword {
    pub kid: u64,
    pub name: String,
    pub example: String,
    pub rule: String
}
/// 订阅模版
pub struct SubTemplate {
    auth_access_token: String,
}
impl SubTemplate {
    pub fn new(_auth_access_token: &str) -> Self {
        SubTemplate {
            auth_access_token: _auth_access_token.to_string(),
        }
    }

    /// 获取模板标题下的关键词列表
    pub async fn get_pub_template_keyword(&self,tid:u32) -> WechatResult<Vec<serde_json::Value>> {
        let uri = format!(
            "{}{}",
            crate::API_DOMAIN,
            format!(
                "/wxaapi/newtmpl/getpubtemplatekeywords?access_token={}&tid={}",
                self.auth_access_token.clone(),
                tid
            )
        );
        let api = Client::new();
        let res = api.get(&uri).await?;
        let data = wechat_sdk::json_decode(&res)?;
        let categories = data["data"].as_array().unwrap();
        let mut list = vec![];
        for c in categories {
            list.push(c.to_owned());
        }
        Ok(list)
    }
    /// 添加模版
    pub async fn add_template(&self,kid:u32,kids:&Vec<u32>,desc:&str) -> WechatResult<String> {
        let uri = format!(
            "{}{}",
            crate::API_DOMAIN,
            format!(
                "/wxaapi/newtmpl/addtemplate?access_token={}",
                self.auth_access_token.clone()
            )
        );
        let api = Client::new();
        let req_data=json!({
            "tid":kid,
            "kidList":kids,
            "sceneDesc":desc
        });
        let res = api.post(&uri, &req_data).await?;
        let data = wechat_sdk::json_decode(&res)?;
        match data["priTmplId"].as_str(){
            Some(s)=>{
                Ok(s.to_owned())
            },
            None=>Err(error!("模版id为空"))
        }
    }

    /// 删除模版
    pub async fn del_template(&self,template_id:String) -> WechatResult<bool> {
        let uri = format!(
            "{}{}",
            crate::API_DOMAIN,
            format!(
                "/wxaapi/newtmpl/deltemplate?access_token={}",
                self.auth_access_token.clone()
            )
        );
        let api = Client::new();
        let req_data=json!({
            "priTmplId":template_id
        });
        let res = api.post(&uri, &req_data).await?;
        wechat_sdk::json_decode(&res)?;
        Ok(true)
    }
}
