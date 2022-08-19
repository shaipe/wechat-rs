//! copyright © ecdata.cn 2022 - present
//!
//! DOC https://developers.weixin.qq.com/doc/offiaccount/Message_Management/Template_Message_Interface.html#1

use crate::API_DOMAIN;
use serde_json::Value;
use wechat_sdk::{json_decode, Client, WechatResult};

pub struct Template {
    access_token: String,
}

impl Template {
    pub fn new(token: &str) -> Self {
        Template {
            access_token: token.to_string(),
        }
    }

    /// 设置所属行业
    /// https://api.weixin.qq.com/cgi-bin/template/api_set_industry?access_token=ACCESS_TOKEN
    pub async fn set_industry(&self, id1: &str, id2: &str) -> WechatResult<Value> {
        let url = format!(
            "{api}/cgi-bin/template/api_set_industry?access_token={token}",
            api = API_DOMAIN,
            token = &self.access_token
        );
        let data = json!({
            "industry_id1": id1,
            "industry_id2": id2
        });
        match Client::new().post(&url, &data).await {
            Ok(res) => json_decode(&res),
            Err(err) => Err(err),
        }
    }

    /// 获取设置的行业信息
    pub async fn get_industry(&self) -> WechatResult<Value> {
        let url = format!(
            "{api}/cgi-bin/template/get_industry?access_token={token}",
            api = API_DOMAIN,
            token = &self.access_token
        );
        match Client::new().get(&url).await {
            Ok(res) => json_decode(&res),
            Err(err) => Err(err),
        }
    }

    /// 获取模板id
    /// param1: 模板短id
    pub async fn get_template_id(&self, short_id: &str) -> WechatResult<Value> {
        let url = format!(
            "{api}/cgi-bin/template/api_add_template?access_token={token}",
            api = API_DOMAIN,
            token = &self.access_token
        );
        match Client::new()
            .post(&url, &json!({ "template_id_short": short_id }))
            .await
        {
            Ok(res) => Ok(json_decode(&res)?["template_id"].clone()),
            Err(err) => Err(err),
        }
    }

    /// 获取设置的行业信息
    /// 直接返回长的模板id字符串型Value
    pub async fn get_list(&self) -> WechatResult<Value> {
        let url = format!(
            "{api}/cgi-bin/template/get_all_private_template?access_token={token}",
            api = API_DOMAIN,
            token = &self.access_token
        );
        match Client::new().get(&url).await {
            Ok(res) => json_decode(&res),
            Err(err) => Err(err),
        }
    }

    /// 删除模板
    /// param1: 模板编Id
    pub async fn delete_template(&self, id: &str) -> WechatResult<Value> {
        let url = format!(
            "{api}/cgi-bin/template/del_private_template?access_token={token}",
            api = API_DOMAIN,
            token = &self.access_token
        );
        match Client::new()
            .post(&url, &json!({ "template_id": id }))
            .await
        {
            Ok(res) => json_decode(&res),
            Err(err) => Err(err),
        }
    }

    /// 发送模板消息
    /// @1-open_id: 接收人的openid
    /// @2-template_id: 模板id
    /// @3-weapp_id: 需要跳转的appid
    /// @4-page_path: 跳转到小程序的页面路径
    /// @5-content_data: 模板消息需要的数据
    pub async fn send_template(
        &self,
        open_id: &str,
        template_id: &str,
        weapp_id: &str,
        page_path: &str,
        content_data: serde_json::Value,
    ) -> WechatResult<Value> {
        let url = format!(
            "{api}/cgi-bin/message/template/send?access_token={token}",
            api = API_DOMAIN,
            token = &self.access_token
        );
        let data = json!({
            "touser": open_id,
            "template_id": template_id,
            // "url":"http://weixin.qq.com/download",
            // "miniprogram":{
            //   "appid": weapp_id,
            //   "pagepath": page_path
            // },
            // "client_msg_id":"MSG_000001",
            "data": content_data
        });
        match Client::new().post(&url, &data).await {
            Ok(res) => json_decode(&res),
            Err(err) => Err(err),
        }
    }

    /// 获取所有的行业信息
    pub async fn get_industries(&self) -> WechatResult<Value> {
        Ok(json!([
            {
                "label": "IT科技",
                "children": [
                    {
                        "label": "互联网/电子商务",
                        "value": 1
                    },
                    {
                        "label": "IT软件与服务",
                        "value": 2
                    },
                    {
                        "label": "IT硬件与设备",
                        "value": 3
                    },
                    {
                        "label": "电子技术",
                        "value": 4
                    },
                    {
                        "label": "通信与运营商",
                        "value": 5
                    },
                    {
                        "label": "网络游戏",
                        "value": 6
                    }
                ]
            },
            {
                "label": "金融业",
                "children": [
                    {
                        "label": "银行",
                        "value": 7
                    },
                    {
                        "label": "基金理财信托",
                        "value": 8
                    },
                    {
                        "label": "保险",
                        "value": 9
                    }
                ]
            },
            {
                "label": "餐饮",
                "children": [
                    {
                        "label": "餐饮",
                        "value": 10
                    }
                ]
            },
            {
                "label": "酒店旅游",
                "children": [
                    {
                        "label": "酒店",
                        "value": 11
                    },
                    {
                        "label": "旅游",
                        "value": 12
                    }
                ]
            },
            {
                "label": "运输与仓储",
                "children": [
                    {
                        "label": "快递",
                        "value": 13
                    },
                    {
                        "label": "物流",
                        "value": 14
                    },
                    {
                        "label": "仓储",
                        "value": 15
                    }
                ]
            },
            {
                "label": "教育",
                "children": [
                    {
                        "label": "培训",
                        "value": 16
                    },
                    {
                        "label": "院校",
                        "value": 17
                    }
                ]
            },
            {
                "label": "政府与公共事业",
                "children": [
                    {
                        "label": "学术科研",
                        "value": 18
                    },
                    {
                        "label": "交警",
                        "value": 19
                    },
                    {
                        "label": "博物馆",
                        "value": 20
                    },
                    {
                        "label": "公共事业非盈利机构",
                        "value": 21
                    }
                ]
            },
            {
                "label": "医药护理",
                "children": [
                    {
                        "label": "医药医疗",
                        "value": 22
                    },
                    {
                        "label": "护理美容",
                        "value": 23
                    },
                    {
                        "label": "保健与卫生",
                        "value": 24
                    }
                ]
            },
            {
                "label": "交通工具",
                "children": [
                    {
                        "label": "汽车相关",
                        "value": 25
                    },
                    {
                        "label": "摩托车相关",
                        "value": 26
                    },
                    {
                        "label": "火车相关",
                        "value": 27
                    },
                    {
                        "label": "飞机相关",
                        "value": 28
                    }
                ]
            },
            {
                "label": "房地产",
                "children": [
                    {
                        "label": "建筑",
                        "value": 29
                    },
                    {
                        "label": "物业",
                        "value": 30
                    }
                ]
            },
            {
                "label": "消费品",
                "children": [
                    {
                        "label": "消费品",
                        "value": 31
                    }
                ]
            },
            {
                "label": "商业服务",
                "children": [
                    {
                        "label": "法律",
                        "value": 32
                    },
                    {
                        "label": "会展",
                        "value": 33
                    },
                    {
                        "label": "中介服务",
                        "value": 34
                    },
                    {
                        "label": "认证",
                        "value": 35
                    },
                    {
                        "label": "审计",
                        "value": 36
                    }
                ]
            },
            {
                "label": "文体娱乐",
                "children": [
                    {
                        "label": "传媒",
                        "value": 37
                    },
                    {
                        "label": "体育",
                        "value": 38
                    },
                    {
                        "label": "娱乐休闲",
                        "value": 39
                    }
                ]
            },
            {
                "label": "印刷",
                "children": [
                    {
                        "label": "印刷",
                        "value": 40
                    }
                ]
            },
            {
                "label": "其它",
                "children": [
                    {
                        "label": "其它",
                        "value": 41
                    }
                ]
            }
        ]))
    }
}
