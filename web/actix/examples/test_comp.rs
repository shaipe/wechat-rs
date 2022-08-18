use wechat::open::{get_tripartite_config, AuthToken as Component, OpenAccount,  Config as TripartiteConfig};
use wechat::weapp::{Category, Code, Basic as Domain, Tester,Privacy,Template};

use std::fs::File;
use std::io;
use std::io::prelude::*;
use wechat_redis::{get_redis_conf, RedisConfig};
use wechat_sdk::{WechatResult,AccessToken};

/// 小程序测试app_id
const _MIN_APP_ID: &str = "wx2be69912728f0108";
/// 公众号app_id
const _OFFICIAL_APP_ID: &str = "wx5dcd5d0fe15c157c";

const _AUDIT_ID: i64 = 460232974;

const _PATH: &str = "pages/mer/tabbar/home";
const OFFICAL_ACCESS_TOKEN_CATCHE_KEY: &str = "OFFICAL_ACCESS_TOKEN_CATCHE_KEY";

fn main() -> io::Result<()> {
    //let access_token = test_offical_app();

    // 小程序授权相关
    let access_token = test_min_app();

    // 设置域名
    //let _=test_set_domain(&access_token);

    // 提交代码
    //let _ = test_commit_code(&access_token);

    // 提交审核
    //let _=test_submit_audit(&access_token);

    // 查看审核状态
    //let _=test_audit_status(&access_token);

    // 查看最近一次审核状态
    //let _=test_latest_audit_status(&access_token);

    // 小程序审核撤回
    //let _=test_undo_audit(&access_token);

    // 体验二维码
    //let _=test_get_qrcode(&access_token);

    // 设置隐私
    //let _=test_set_privacy(&access_token);

    // 解绑体验者
    //let _=test_unbind_tester(&access_token);

    // 生成小程序码
    //let _=test_get_wxa_code(&access_token);

    //let _ = test_release(&access_token);
    // 创建开放平台帐号
    //let _=test_create_open(&access_token);

    // 绑定开放平台帐号
    //let _ = test_bind_open(&access_token);

    // 解绑开放平台帐号
    //let _=test_unbind_open(&access_token);

    Ok(())
}

/// 测试公众号
#[allow(dead_code)]
fn test_offical_app()->String {
    let tripart_config: TripartiteConfig = get_tripartite_config();
    let redis_config: RedisConfig = get_redis_conf();
    let comp = Component::new(tripart_config.clone());
    let redis_con = format!(
        "redis://:{}@{}:{}/{}",
        &redis_config.password, &redis_config.server, &redis_config.port, redis_config.dbid
    );

    let token=get_comp_token(&redis_con,&tripart_config.app_id);
    // //获取详情
    // let rs=actix_rt::System::new().block_on(comp.fetch_authorizer_info(_OFFICIAL_APP_ID));
    // println!("===={:?}",rs);

    // //获取授权信息
    // let code="queryauthcode@@@qW8POq0hjqlMoP4rZKbQ0YLWYH_Xx8OOODw4U97cSS9gdWUwG8v5Etgk-37lQWEEz48ZHFYn-gWUH7qp5MVBRQ";

    // let rs=actix_rt::System::new().block_on(comp.query_auth(code));
    // println!("==={:?}",rs);

    //获取授权详情
    // let refresh_token = "refreshtoken@@@WLdb4MPANDWj71mXhYsl0OB3n6CweoPxsTLB2eK3M2M";
    // let rs = actix_rt::System::new()
    //     .block_on(comp.fetch_authorizer_token(_OFFICIAL_APP_ID, refresh_token,&token.0));
    // println!("fetch_authorizer_token==={:?}", rs);

    let authorizer_token="55_-w7CYvV-4KtYUvf_ZRu3Cwb0eCxDxTvAV0gD1hyKX-idmRtUi0vEmFm_3g_7dCXs897bgKT_UNomrIJiVod-boSeRK3BjbqhRss3GlLhilxnIabcpXT_Z-nC-9Bdl5L5LOFe0UkKlRd925KVYVHfAEDMJV";
    // let rs = actix_rt::System::new().block_on(comp.get_template_list(None,&token.0));
    // println!("==={:?}", rs);
    authorizer_token.to_owned()
}

#[allow(dead_code)]
/// 测试小程序
fn test_min_app() -> String {
    let tripart_config: TripartiteConfig = get_tripartite_config();
    println!("tripart_config={:?}",tripart_config);
    let redis_config: RedisConfig = get_redis_conf();
    let comp = Component::new(tripart_config.clone());

    let redis_con = format!(
        "redis://:{}@{}:{}/{}",
        &redis_config.password, &redis_config.server, &redis_config.port, redis_config.dbid
    );

    let token=get_comp_token(&redis_con,&tripart_config.app_id);
    println!("{:?}",token);
    // //获取详情
    // let rs=actix_rt::System::new().block_on(comp.fetch_authorizer_info(_MIN_APP_ID));
    // println!("===={:?}",rs);

    // let code="queryauthcode@@@OC4bbkN1COoHhO-mJzcCBIIt5mKg8k73wN-mYx_17T5BWeDeTHSZzhDDeeAlLs3_47D_hqeAYFcPZV0Q-kR0Xg";
    // let rs=actix_rt::System::new().block_on(comp.query_auth(code));
    // println!("==={:?}",rs);

    // //获取授权令牌
    let refresh_token="refreshtoken@@@yzx1x8n6ECinXW2iBwTD-804NWNCXfGdCbxLxSz0VqA";
    let rs=actix_rt::System::new().block_on(comp.fetch_authorizer_token(_MIN_APP_ID,refresh_token,&token.0));
    println!("fetch_authorizer_token==={:?}",rs);
    
    set_official_access_token(&redis_con,_MIN_APP_ID,rs.unwrap());
    let authorizer_token="55_9tyg8iY42xIyGl9ayJdUSZwnWVHy0id_UKgxIyUa2oKnjKNnFRUpP7aZ9u8s5jUMWX2Kjx-NIKRiaAHVzv-TylDCGaPs0BST_RIjEiOSxc3jAA69jffjxuWckPcGJQ1b6ooz6NTU1CP_qqQjIHReAFDQPZ";
    
    // let rs=actix_rt::System::new().block_on(Template::new(&token.0).get_template_list(Some(0)));
    // println!("===res{:?}",rs.unwrap()[0].to_string());

    authorizer_token.to_owned()
}
#[allow(dead_code)]
/// 设置域名
fn test_set_domain(access_token: &str) -> WechatResult<u64> {
    let min_domain = Domain::new(access_token);

    let mut req_domain = vec![];
    req_domain.push("https://wechat.ecdata.cn".to_owned());
    req_domain.push("https://assets.ecdata.cn".to_owned());
    req_domain.push("https://ht.ecdata.cn".to_owned());
    let rs = actix_rt::System::new().block_on(min_domain.set_server_domain(
        req_domain.clone(),
        req_domain.clone(),
        req_domain.clone(),
        req_domain.clone(),
    ))?;
    println!("==={:?}", rs);
   
    //futures::executor::block_on(min_domain.set_webview_domain(req_domain.clone()));
    let rs = actix_rt::System::new().block_on(min_domain.set_webview_domain(req_domain.clone()))?;
    println!("==={:?}", rs);

    Ok(0)
}
#[allow(dead_code)]
/// 上传代码
fn test_set_privacy(access_token: &str) -> WechatResult<u64> {
    // 加载配置文件
    let file_path = "config/privacy.json";

    // 打开文件
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            panic!("no such file {} exception: {}", file_path, e)
        }
    };
    // 读取文件到字符串变量
    let mut privacy_json = String::new();
    match file.read_to_string(&mut privacy_json) {
        Ok(s) => s,
        Err(e) => {
            panic!("Error Reading file:{}", e);
        }
    };
    let privacy_json_v: serde_json::Value = serde_json::from_str(&privacy_json).unwrap();

    let mincode = Privacy::new(access_token);
    let rs = actix_rt::System::new().block_on(mincode.set_privacy(privacy_json_v));
    println!("==={:?}", rs);
    Ok(0)
}
#[allow(dead_code)]
/// 上传代码
fn test_commit_code(access_token: &str) -> WechatResult<u64> {
    // 加载配置文件
    let file_path = "config/ext.json";

    // 打开文件
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            panic!("no such file {} exception: {}", file_path, e)
        }
    };
    // 读取文件到字符串变量
    let mut ext_json = String::new();
    match file.read_to_string(&mut ext_json) {
        Ok(s) => s,
        Err(e) => {
            panic!("Error Reading file:{}", e);
        }
    };
    let ext_json_v: serde_json::Value = serde_json::from_str(&ext_json).unwrap();

    //println!("ext_json_v={:?}",ext_json_v);
    let mincode = Code::new(access_token);
    let rs = actix_rt::System::new().block_on(mincode.commit_code(
        "3",
        ext_json_v,
        "1.0.2",
        "测试提交2.0",
    ));
    println!("==={:?}", rs);
    Ok(0)
}
#[allow(dead_code)]
/// 提交审核
pub fn test_submit_audit(access_token: &str) -> WechatResult<u64> {
    let category = Category::new(access_token);

    let c_list = actix_rt::System::new().block_on(category.get_category())?;

    let mut item = c_list[0].clone();
    item.address = _PATH.to_owned();
    item.title = "谷物".to_owned();
    item.tag = "女装".to_string();
    let mincode = Code::new(access_token);
    let rs = actix_rt::System::new().block_on(mincode.submit_audit(item));
    println!("==={:?}", rs);
    Ok(0)
}
#[allow(dead_code)]
/// 获取体验二维码
pub fn test_get_qrcode(access_token: &str) -> WechatResult<u64> {
    let mincode = Code::new(access_token);
    let rs = actix_rt::System::new().block_on(mincode.get_qrcode(_PATH))?;

    //println!("{:?}",rs);
    Ok(0)
}
#[allow(dead_code)]
/// 查看审核状态
pub fn test_audit_status(access_token: &str) -> WechatResult<u64> {
    let mincode = Code::new(access_token);
    let rs = actix_rt::System::new().block_on(mincode.audit_status(_AUDIT_ID));
    println!("==={:?}", rs);
    Ok(0)
}
#[allow(dead_code)]
/// 查询指定版本的审核状态
pub fn test_latest_audit_status(access_token: &str) -> WechatResult<u64> {
    let mincode = Code::new(access_token);
    let rs = actix_rt::System::new().block_on(mincode.latest_audit_status());
    println!("==={:?}", rs);
    Ok(0)
}
#[allow(dead_code)]
/// 小程序审核撤回
pub fn test_undo_audit(access_token: &str) -> WechatResult<u64> {
    let mincode = Code::new(access_token);
    let rs = actix_rt::System::new().block_on(mincode.undo_audit());
    println!("==={:?}", rs);
    Ok(0)
}
#[allow(dead_code)]
/// 发布小程序
pub fn test_release(access_token: &str) -> WechatResult<u64> {
    let mincode = Code::new(access_token);
    let rs = actix_rt::System::new().block_on(mincode.release());
    println!("==={:?}", rs);
    Ok(0)
}
#[allow(dead_code)]
/// 绑定体验者
pub fn test_bind_tester(access_token: &str) -> WechatResult<u64> {
    let wechat_id = "chen-9-1-8";
    let bll = Tester::new(access_token);
    let rs = actix_rt::System::new().block_on(bll.bind_tester(wechat_id))?;

    println!("==={:?}", rs);
    Ok(0)
}

#[allow(dead_code)]
/// 解绑体验者
pub fn test_unbind_tester(access_token: &str) -> WechatResult<u64> {
    let wechat_id = "chen-9-1-8";
    let bll = Tester::new(access_token);
    let rs = actix_rt::System::new().block_on(bll.unbind_tester(wechat_id))?;

    println!("==={:?}", rs);
    Ok(0)
}

#[allow(dead_code)]
/// 解绑体验者
pub fn test_get_wxa_code(access_token: &str) -> WechatResult<u64> {
    let bll = Code::new(access_token);
    let rs = actix_rt::System::new().block_on(bll.get_wxa_code(_PATH, 430, false, "", false))?;

    //println!("==={:?}", rs);
    Ok(0)
}

#[allow(dead_code)]
/// 创建开放平台帐号
pub fn test_create_open(access_token: &str) -> WechatResult<u64> {
    let bll = OpenAccount::new(_MIN_APP_ID, access_token);
    let rs = actix_rt::System::new().block_on(bll.create_open())?;

    println!("==={:?}", rs);
    Ok(0)
}
/// 绑定开放平台帐号
pub fn test_bind_open(access_token: &str) -> WechatResult<u64> {
    let open_id = "wx2454d0ced8d53230";
    let bll = OpenAccount::new(_MIN_APP_ID, access_token);
    let rs = actix_rt::System::new().block_on(bll.bind_open(open_id))?;

    println!("==={:?}", rs);
    Ok(0)
}
#[allow(dead_code)]
/// 绑定开放平台帐号
pub fn test_unbind_open(access_token: &str) -> WechatResult<u64> {
    let open_id = "wx2454d0ced8d53230";
    let bll = OpenAccount::new(_MIN_APP_ID, access_token);
    let rs = actix_rt::System::new().block_on(bll.unbind_open(open_id))?;

    println!("==={:?}", rs);
    Ok(0)
}
const COMP_CATCHE_KEY: &str = "COMP_ACCESS_TOKEN_CATCHE_KEY";
use wechat_redis::{RedisStorage, SessionStore};
/// 获取
pub fn get_comp_token(redis_con: &str, key: &str) -> (String, u64) {
    let cache_key = format!("{0}_{1}", COMP_CATCHE_KEY, key);

    match RedisStorage::from_url(format!("{}", redis_con)) {
        Ok(session) => {
            let d = "".to_owned();
            if let Some(v) = session.get(cache_key, "get".to_owned(), Some(d)) {
                let arr: Vec<_> = v.split('|').collect();
                if arr.len() == 2 {
                    return (arr[0].to_string(), arr[1].parse::<u64>().unwrap());
                }
                return ("".to_owned(), 0);
            } else {
                return ("".to_owned(), 0);
            }
        }
        Err(_) => {
            return ("".to_owned(), 0);
        }
    }
}

pub fn set_official_access_token(redis_con: &str,key: &str, cnf: (String, u64)) {
    let url = format!("{}", redis_con);
    let cache_key = format!("{0}_{1}", OFFICAL_ACCESS_TOKEN_CATCHE_KEY, key);
    match RedisStorage::from_url(url) {
        Ok(session) => {
            session.set(cache_key, format!("{}|{}", cnf.0, cnf.1), Some(2 * 55 * 60));
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}