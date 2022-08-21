//! copyright © ecdata.cn 2021 - present
//! 小程序分类管理

use serde_json::Value;
use wechat_sdk::{json_decode, Client, WechatResult};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const API_DOMAIN: &'static str = "https://api.weixin.qq.com";
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CategoryItem {
    pub address: String,
    pub tag: String,
    pub first_class: String,
    pub second_class: String,
    pub first_id: i64,
    pub second_id: i64,
    pub title: String,
    pub desc: String,
}
pub struct Category {
    auth_access_token: String,
}
impl Category {
    pub fn new(_auth_access_token: &str) -> Self {
        Category {
            auth_access_token: _auth_access_token.to_string(),
        }
    }

    /// 获取可设置的所有类目
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/category-management/getAllCategories.html
    pub async fn get_all_categories(&self) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/cgi-bin/wxopen/getallcategories?access_token={}",
                self.auth_access_token.clone()
            )
        );
        let res = Client::new().get(&uri).await?;
        json_decode(&res)
    }

    /// 获取已设置的所有类目
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/category-management/getSettingCategories.html
    pub async fn get_category(&self) -> WechatResult<Vec<CategoryItem>> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/cgi-bin/wxopen/getcategory?access_token={}",
                self.auth_access_token.clone()
            )
        );

        let hash: HashMap<String, String> = HashMap::new();
        let res = Client::new().post(&uri, &hash).await?;
        let data = wechat_sdk::json_decode(&res)?;
        let categories = data["categories"].as_array().unwrap();
        let mut list = vec![];
        for c in categories {
            list.push(CategoryItem {
                address: "".to_owned(),
                tag: "".to_owned(),
                first_id: c["first"].as_i64().unwrap_or_default(),
                first_class: c["first_name"].to_string(),
                second_id: c["second"].as_i64().unwrap_or_default(),
                second_class: c["second_name"].to_string(),
                title: "".to_owned(),
                desc: format!(
                    "{}->{}",
                    c["first_name"].to_string(),
                    c["second_name"].to_string()
                ),
            });
        }
        Ok(list)
    }

    /// 获取不同类型主体可设置的类目
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/category-management/getAllCategoriesByType.html
    /// @param1 - verify_type : 如果不填，默认传0；个人主体是0；企业主体是1；政府是2；媒体是3；其他组织是4
    pub async fn get_category_by_type(&self, verify_type: i8) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/cgi-bin/wxopen/getcategoriesbytype?access_token={}",
                self.auth_access_token.clone()
            )
        );

        let data = json!(
        {
            "verify_type": verify_type
        });

        let res = Client::new().post(&uri, &data).await?;

        json_decode(&res)
    }

    /// 添加类目
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/category-management/addCategory.html
    pub async fn add_category(&self) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/cgi-bin/wxopen/addcategory?access_token={}",
                self.auth_access_token.clone()
            )
        );

        let data = json!({});

        let res = Client::new().post(&uri, &data).await?;

        json_decode(&res)
    }

    /// 删除目录
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/category-management/deleteCategory.html
    /// @param1 - first	number	是	一级类目 ID
    /// @param2 - second	number	是	二级类目 ID
    pub async fn delete_category(&self, first: i64, second: i64) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/cgi-bin/wxopen/deletecategory?access_token={}",
                self.auth_access_token.clone()
            )
        );

        let data = json!(
        {
            "first": first,
            "second": second
        });

        let res = Client::new().post(&uri, &data).await?;

        json_decode(&res)
    }

    /// 修改类目资质信息
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/category-management/modifyCategory.html
    pub async fn modify_category(&self) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/cgi-bin/wxopen/modifycategory?access_token={}",
                self.auth_access_token.clone()
            )
        );

        let data = json!({});

        let res = Client::new().post(&uri, &data).await?;

        json_decode(&res)
    }

    /// 获取类目名称信息
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/category-management/getAllCategoryName.html
    pub async fn get_category_names(&self) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/get_category?access_token={}",
                self.auth_access_token.clone()
            )
        );
        let res = Client::new().get(&uri).await?;
        json_decode(&res)
    }
}
