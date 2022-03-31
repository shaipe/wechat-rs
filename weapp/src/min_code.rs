//! copyright © ecdata.cn 2021 - present
//! 小程序代码管理

use wechat_sdk::{Client, WechatResult};

use std::{collections::HashMap, fmt::format};

use crate::min_category::MinCategoryItem;

const API_DOMAIN: &'static str = "https://api.weixin.qq.com";

pub struct MinCode {
    authorizer_access_token: String,
}
impl MinCode {
    pub fn new(_authorizer_access_token: &str) -> Self {
        MinCode {
            authorizer_access_token: _authorizer_access_token.to_string(),
        }
    }

    /// 上传代码
    pub async fn commit_code(
        &self,
        template_id: &str,
        ext_json: serde_json::Value,
        user_version: &str,
        user_desc: &str,
    ) -> WechatResult<serde_json::Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/commit?access_token={}",
                self.authorizer_access_token.clone()
            )
        );
        
      
             
        let ext_json_str=ext_json.to_string();
        //let ext_json_str="{\"extAppid\":\"wx2be69912728f0108\",\"ext\":{\"attr1\":\"value1\",\"attr2\":\"value2\"},\"window\":{}}";
        let data_str=json!(
        {
            "template_id":template_id,
            "ext_json":ext_json_str,
            "user_version":user_version,
            "user_desc":user_desc
        }).to_string();
        println!("ext_json_str={}",data_str);
       
        let api = Client::new();
        let res = api.post(&uri, &data_str).await?;
        println!("res==={:?}",res);
        wechat_sdk::json_decode(&res)
    }

    /// 提交审核
    pub async fn submit_audit(
        &self,
        item: MinCategoryItem,
    ) -> WechatResult<serde_json::Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/commit?access_token={}",
                self.authorizer_access_token.clone()
            )
        );
        let mut list=vec![];
        list.push(item);
        let s=serde_json::to_value(list).unwrap();
        let api = Client::new();
        let res = api.post(&uri, &s).await?;
        wechat_sdk::json_decode(&res)
    }
}
