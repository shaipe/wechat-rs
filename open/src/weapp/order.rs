//! copyright © ecdata.cn 2022 - present
//! 购物订单
//!
//!

use serde_json::Value;
use wechat_sdk::{json_decode, Client, WechatResult};

use crate::API_DOMAIN;

pub struct Order {
    auth_access_token: String,
}

impl Order {
    /// 创建对象
    pub fn new(auth_access_token: String) -> Self {
        Order {
            auth_access_token: auth_access_token,
        }
    }

    /// 上传购物详情
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/shopping-orders/uploadShoppingInfo.html
    pub async fn upload_order_info(&self) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/user-order/orders?access_token={}",
                self.auth_access_token.clone()
            )
        );

        let data = json!(
        {
        });

        let res = Client::new().post(&uri, &data).await?;

        json_decode(&res)
    }


    /// 上传物流信息
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/shopping-orders/uploadShippingInfo.html
    pub async fn upload_delivery_info(&self) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/user-order/orders/shippings?access_token={}",
                self.auth_access_token.clone()
            )
        );

        let data = json!(
        {
        });

        let res = Client::new().post(&uri, &data).await?;

        json_decode(&res)
    }

    /// 上传合单购物详情
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/shopping-orders/uploadCombinedShoppingInfo.html
    pub async fn upload_combine_order(&self) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/user-order/combine-orders?access_token={}",
                self.auth_access_token.clone()
            )
        );

        let data = json!(
        {
        });

        let res = Client::new().post(&uri, &data).await?;

        json_decode(&res)
    }

    /// 上传合单物流信息
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/shopping-orders/uploadCombinedShippingInfo.html 
    pub async fn upload_combine_delivery(&self) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/user-order/combine-orders/shippings?access_token={}",
                self.auth_access_token.clone()
            )
        );

        let data = json!(
        {
        });

        let res = Client::new().post(&uri, &data).await?;

        json_decode(&res)
    }

    /// 开通购物订单产品权限
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/shopping-orders/openShoppingOrderProductPermission.html
    pub async fn open_order_permission(&self) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/user-order/orders-permission/open?access_token={}",
                self.auth_access_token.clone()
            )
        );

        let data = json!(
        {
        });

        let res = Client::new().post(&uri, &data).await?;

        json_decode(&res)
    }

    /// 提交购物订单接入审核
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/shopping-orders/confirmProductPermission.html
    pub async fn audit_order_permission(&self) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/user-order/orders-permission/confirm?access_token={}",
                self.auth_access_token.clone()
            )
        );

        let data = json!(
        {
        });

        let res = Client::new().post(&uri, &data).await?;

        json_decode(&res)
    }

    /// 验证购物订单上传结果
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/shopping-orders/ShoppingInfoVerifyUploadResult.html
    pub async fn verify_orders(&self) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/user-order/shoppinginfo/verify?access_token={}",
                self.auth_access_token.clone()
            )
        );

        let data = json!(
        {
        });

        let res = Client::new().post(&uri, &data).await?;

        json_decode(&res)
    }

}
