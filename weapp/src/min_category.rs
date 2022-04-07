//! copyright © ecdata.cn 2021 - present
//! 小程序分类管理

use wechat_sdk::{Client, WechatResult};

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

const API_DOMAIN: &'static str ="https://api.weixin.qq.com";
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MinCategoryItem{
    pub address:String,
    pub tag:String,
    pub first_class:String,
    pub second_class:String,
    pub first_id:i64,
    pub second_id:i64,
    pub title:String,
    pub desc:String,
}
pub struct MinCategory {
    authorizer_access_token: String,
}
impl MinCategory {

    pub fn new( _authorizer_access_token: &str) -> Self {
        MinCategory {
            authorizer_access_token: _authorizer_access_token.to_string(),
        }
    }
    
    /// 获取小程序分类
    pub async fn get_category(&self)->WechatResult<Vec<MinCategoryItem>>{
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/cgi-bin/wxopen/getcategory?access_token={}",
                self.authorizer_access_token.clone()
            )
        );
        let api = Client::new();
        let hash:HashMap<String,String>=HashMap::new();
        let res = api.post(&uri, &hash).await?;
       let data=wechat_sdk::json_decode(&res)?;
       let categories=data["categories"].as_array().unwrap();
       let mut list=vec![];
       for c in categories{
            list.push(MinCategoryItem{
                address:"".to_owned(),
                tag:"".to_owned(),
                first_id:c["first"].as_i64().unwrap_or_default(),
                first_class:c["first_name"].to_string(),
                second_id:c["second"].as_i64().unwrap_or_default(),
                second_class:c["second_name"].to_string(),
                title:"".to_owned(),
                desc:format!("{}->{}",c["first_name"].to_string(),c["second_name"].to_string())
            });
       }
       Ok(list)
    }
}
