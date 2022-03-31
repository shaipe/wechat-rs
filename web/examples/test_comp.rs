use wechat::open::{get_tripartite_config, Component, TripartiteConfig};
use wechat::weapp::{MinCategory, MinCategoryItem, MinCode, MinDomain};

use std::fs::File;
use std::io;
use std::io::prelude::*;
use wechat_redis::{get_redis_conf, RedisConfig};
use wechat_sdk::WechatResult;

fn main() -> io::Result<()> {
    //test_offical_app();
    let access_token = test_min_app();
    //let _=test_set_domain(&access_token);
    let _ = test_commit_code(&access_token);
    //let _=test_submit_audit(&access_token);
    Ok(())
}
/// 测试公众号
fn test_offical_app() {
    let tripart_config: TripartiteConfig = get_tripartite_config();
    let redis_config: RedisConfig = get_redis_conf();
    let comp = Component::new(tripart_config.clone(), redis_config.clone());

    let app_id = "wx999317f16de96ce3";

    // //获取详情
    // let rs=actix_rt::System::new().block_on(comp.fetch_authorizer_info(app_id));
    // println!("===={:?}",rs);

    // //获取授权信息
    // let code="queryauthcode@@@qW8POq0hjqlMoP4rZKbQ0YLWYH_Xx8OOODw4U97cSS9gdWUwG8v5Etgk-37lQWEEz48ZHFYn-gWUH7qp5MVBRQ";

    // let rs=actix_rt::System::new().block_on(comp.query_auth(code));
    // println!("==={:?}",rs);

    //获取授权详情
    let refresh_token = "refreshtoken@@@WLdb4MPANDWj71mXhYsl0OB3n6CweoPxsTLB2eK3M2M";
    let rs = actix_rt::System::new().block_on(comp.fetch_authorizer_token(app_id, refresh_token));
    println!("fetch_authorizer_token==={:?}", rs);

    let authorizer_token="55_4MoCgiD3ni28AXxeaqwTMpPMoYUYW5M0wqIHBtTn9AUTESGdly1wm2_55UF2C2a3RMWESSPB83XduKY8iDy2Cs4tSSzQiFl-X3RcWalNfeUAy89EtI-z46Gb7gnXEJsr7DAF3dSuL0tyufeMVSChAEADAK";
    let rs = actix_rt::System::new().block_on(comp.get_template_list(None));
    println!("==={:?}", rs);
}

/// 测试小程序
fn test_min_app() -> String {
    let tripart_config: TripartiteConfig = get_tripartite_config();
    let redis_config: RedisConfig = get_redis_conf();
    let comp = Component::new(tripart_config.clone(), redis_config.clone());

    let app_id = "wx2be69912728f0108";

    // //获取详情
    // let rs=actix_rt::System::new().block_on(comp.fetch_authorizer_info(app_id));
    // println!("===={:?}",rs);

    // let code="queryauthcode@@@OC4bbkN1COoHhO-mJzcCBIIt5mKg8k73wN-mYx_17T5BWeDeTHSZzhDDeeAlLs3_47D_hqeAYFcPZV0Q-kR0Xg";
    // let rs=actix_rt::System::new().block_on(comp.query_auth(code));
    // println!("==={:?}",rs);

    // //获取授权令牌
    // let refresh_token="refreshtoken@@@68i490TBYPIBf3dnYuvWD42Vy7TzvbfwJg88t6FQSPg";
    // let rs=actix_rt::System::new().block_on(comp.fetch_authorizer_token(app_id,refresh_token));
    // println!("fetch_authorizer_token==={:?}",rs);

    let authorizer_token="55_97kc7FHIy8wtfkiQW__6c0v9PdcnQ2M1ZCYc1QfXEUkp8YxZSFfQekFPTnUuI9MX52ohuEYEwfVgX6ClqkK7C9HaKvSxLD8wmjIyeb_W0xBe5QX_upE0prtvntWaS-eM3XZjqTHUYPznb8K-JPOdALDCIH";
    let rs=actix_rt::System::new().block_on(comp.get_template_list(Some(1)));
    println!("==={:?}",rs);

    authorizer_token.to_owned()
}
/// 设置域名
fn test_set_domain(access_token: &str) -> WechatResult<u64> {
    let min_domain = MinDomain::new(access_token);

    let mut req_domain = vec![];
    req_domain.push("https://wechat.ecdata.cn".to_owned());
    req_domain.push("https://ecdata.cn".to_owned());

    let rs = actix_rt::System::new().block_on(min_domain.set_server_domain(
        req_domain.clone(),
        req_domain.clone(),
        req_domain.clone(),
        req_domain.clone(),
    ))?;
    println!("==={:?}", rs);

    let rs = actix_rt::System::new().block_on(min_domain.set_webview_domain(req_domain.clone()))?;
    println!("==={:?}", rs);

    Ok(0)
}
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
    let mincode = MinCode::new(access_token);
    let rs =
        actix_rt::System::new().block_on(mincode.commit_code("1", ext_json_v, "1.0.0", "测试提交"));
    println!("==={:?}", rs);
    Ok(0)
}
/// 提交审核
pub fn test_submit_audit(access_token: &str) -> WechatResult<u64> {
    let category = MinCategory::new(access_token);

    let c_list = actix_rt::System::new().block_on(category.get_category())?;

    let mut item = c_list[0].clone();
    item.address = "login/login".to_owned();
    item.title = "谷物".to_owned();

    let mincode = MinCode::new(access_token);
    let rs = actix_rt::System::new().block_on(mincode.submit_audit(item));
    println!("==={:?}", rs);
    Ok(0)
}
