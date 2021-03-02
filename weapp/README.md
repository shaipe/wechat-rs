# 微信小程序接口对接



##  微信官方WIKI

https://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1474632113_xQVCl&token=&lang=zh_CN

1. [小程序开发文档](https://mp.weixin.qq.com/debug/wxadoc/dev/index.html)

2. [小程序设计指南](https://mp.weixin.qq.com/debug/wxadoc/design/index.html)

3. [小程序开发工具](https://mp.weixin.qq.com/debug/wxadoc/dev/devtools/download.html)

4. [后端开发文档](https://developers.weixin.qq.com/miniprogram/dev/api-backend/)

   直接下载（`此程序为微信官方提供，不需要进行破解、运行0.7版本等繁琐步骤。`）：

   [windows 64](https://servicewechat.com/wxa-dev-logic/download_redirect?type=x64&from=mpwiki&t=1474644089434)

   [windows 32](https://servicewechat.com/wxa-dev-logic/download_redirect?type=ia32&from=mpwiki&t=1474644089434)

   [mac](https://servicewechat.com/wxa-dev-logic/download_redirect?type=darwin&from=mpwiki&t=1474644089434)


#### 1、小程序登录

```
接口：/sns/component/jscode2session?appid=APPID&js_code=JSCODE&grant_type=authorization_code&component_appid=COMPONENT_APPID&component_access_token=COMPONENT_ACCESS_TOKEN

appid 小程序的 AppID
js_code	string	wx.login 获取的 code
grant_type authorization_code
component_appid	第三方平台 appid
component_access_token	第三方平台 access_token

```