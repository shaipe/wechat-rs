# wechat-rs

> 微信公众号、开放平台、小程序、小商店、微信支付,企业微信接口，采用Rust语言的对接实现接口对接的sdk，方便使用Rust语言进行微信开发的同学开箱即用。

## 子模块

> 针对每个子模块进行整体性说明
> 子模块采用features的方式进行设计规划,可以整体使用也可以独立使用按项目的需要进行引入.

例如只需要使用小程序模块,在``cargo.toml``中引入时添加features

```toml
wechat = {version="0.1.0", features=["weapp"]}
```

### 公众号

### 开放平台

### 小程序

### 微信小店

### 微信支付

### 企业微信

只使用一个第三方平台，小程序发版也只有一个模板；


## 布署
```
1、修改第三方里面的域名配置，并全网发布
2、待全网发布成功后
1）、执行 
-- api 请求域名
2）、更新extjson并保存第三方配置
3、布署时，布署几套，而且每套服务器分布式布署

需要测试的点

检测第三方access_token是否共用，已经授权过的客户能正常使用；
后台公众号/小程序授权；
小程序发布；
网页授权；
以及检测h5微信相关的操作；

```