//! copyright © ecdata.cn 2022 - present
//! 商户进件申请
//! created by shaipe
//!

use wechat_sdk::{Client, WechatResult};

use crate::API_DOMAIN;

pub struct Apply;

impl Apply {
    /// 申请进件
    /// https://pay.weixin.qq.com/wiki/doc/apiv3_partner/apis/chapter11_1_1.shtml
    ///
    pub async fn apply_partner(&self) -> WechatResult<serde_json::Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!("/cgi-bin/component/setprivacysetting?access_to",)
        );

        let data = json!({
            "business_code" : "business_code",
            "contact_info": {
                "contact_name" :"contact_name",
                "contact_id_number" :"contact_id_number",
                "mobile_phone" :"mobile_phone",
                "contact_email" :"contact_email"
            },
            "subject_info" : {
                "subject_type" : "subject_type",
                "business_license_info" : {
                    "license_copy" : "license_copy",
                    "license_number" : "license_number",
                    "merchant_name" : "merchant_name",
                    "legal_person" : "legal_person"
                },
                "identity_info" : {
                    "id_doc_type" : "id_doc_type",
                    "id_card_info" : {
                        "id_card_copy" : "id_card_copy",
                        "id_card_national" : "id_card_national",
                        "id_card_name" :"id_card_name",
                        "id_card_number" :"id_card_number",
                        "card_period_begin" : "card_period_begin",
                        "card_period_end" : "card_period_end",
                    "owner" :  true
                    }
                },
            "business_info" : {
                "merchant_shortname" : "merchant_shortname",
                "service_phone" : "service_phone",
                "sales_info" : {
                    "sales_scenes_type" :
                        "sales_scenes_type"
                },
                    "biz_store_info" : {
                        "biz_store_name" : "biz_store_name",
                        "biz_address_code" : "biz_address_code",
                        "biz_store_address" : "biz_store_address",
                        "store_entrance_pic" :
                            "store_entrance_pic"
                        ,
                        "indoor_pic" :
                            "indoor_pic"

                    }

            },
            "settlement_info" : {
                "settlement_id" : "settlement_id",
                "qualification_type" : "qualification_type",
                "qualifications" :  "qualifications",
                "activities_id" : "20191030111cff5b5e" ,
                "activities_rate" : "activities_rate"
        },
            "bank_account_info" : {
                "bank_account_type" : "bank_account_type",
                "account_name" :"account_name",
                "account_bank" : "account_bank",
                "bank_address_code" : "bank_address_code",
                "bank_name" : "bank_name",
                "account_number" :"account_number"
        }

        }});

        let res = Client::new().post(&uri, &data).await?;
        wechat_sdk::json_decode(&res)
    }
}
