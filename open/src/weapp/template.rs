use serde_json::Value;
use wechat_sdk::{Client, WechatResult};

use crate::API_DOMAIN;

/// 
pub struct Template {
    /// 
    comp_access_token: String
}

impl Template {

    /// 
    pub fn new(access_token: &str) -> Self {
        Template {
            comp_access_token: access_token.to_owned()
        }
    }

    // 获取代码草稿列表
    // https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/ThirdParty/code_template/gettemplatedraftlist.html

    // 将草稿添加到代码模板库
    // https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/ThirdParty/code_template/addtotemplate.html

    /// 获取模版列表
    /// https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/ThirdParty/code_template/gettemplatelist.html
    pub async fn get_template_list(
        &self,
        template_type: Option<i32>
    ) -> WechatResult<Vec<serde_json::Value>> {
        // 获取
        let mut uri = format!(
            "{}/wxa/gettemplatelist?access_token={}",
            API_DOMAIN, self.comp_access_token
        );
        if let Some(t) = template_type {
            uri = format!("{}&template_type={}", uri, t);
        }
        // get
        let res = Client::new().get(&uri).await?;
        let data = crate::parse_json(&res).await?;
       
        let mut list: Vec<Value> = vec![];
        let list_temp =match data["template_list"].as_array(){
            Some(s)=>s.to_owned(),
            None=>vec![]
        };
        for a in list_temp.iter() {
            let mut v: serde_json::map::Map<std::string::String, serde_json::value::Value> =
                serde_json::map::Map::new();
            let template_id = a["template_id"].as_i64().unwrap();
            let user_version = a["user_version"].as_str().unwrap();
            let user_desc = a["user_desc"].as_str().unwrap();
            v.insert(
                "template_id".to_owned(),
                Value::String(template_id.to_string()),
            );
            v.insert(
                "user_version".to_owned(),
                Value::String(user_version.to_string()),
            );
            v.insert("user_desc".to_owned(), Value::String(user_desc.to_string()));
            list.push(serde_json::to_value(v).unwrap());
        }

        Ok(list)
    }

    // 删除指定代码模板
    // https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/ThirdParty/code_template/deletetemplate.html
}
