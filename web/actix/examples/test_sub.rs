use wechat_weapp::{SubTemplate, SubTemplateKeyword, Subscribe};

use std::fs::File;
use std::io;
use std::io::prelude::*;
use wechat_redis::{get_redis_conf, RedisConfig};
use wechat_sdk::{AccessToken, WechatResult};
use serde_json;

fn main() -> io::Result<()> {
    // 获取配置
    //access_token
    let access_token=get_access_token("wx455639023de66adb","");
    println!("access_token={}",access_token);
    let tid=3578;
    get_pub_template_keyword(&access_token,tid);
    let kids=vec![1,3,2,4];
    let template_id=add_template(&access_token,tid,&kids,"订单支付成功");

    let data=serde_json::json!({
        "thing1":{
            "value":"可口可乐"
        },
        "character_string3":{
            "value":"ts75220191001"
        },
        "amount2":{
            "value":180
        },
        "time4":{
            "value":"2022-06-07"
        }
        
    });
    send_sub_message(&access_token,"oUpe_5Wd1OPFphq10PaByGYPsioM",&template_id,&data,"home/search/search");
    Ok(())
}
fn get_access_token(app_id: &str, secret: &str) -> String {
    return "57_4Vu6tesJqDbtQUNmPVA5OvcqTSyGj-5nKc5zGAK8pVJjEOMHQVFQUiOHqDzJZPrRSyp_RjwdGtyJ_hrrI6fGB5F7BUdYe_4ryK_vDTgp19G2IqREyGz3pq2z70vowUlL8CMWqCuZrUsn6lJlIRDdAIAXGS".to_owned();
    let rs = actix_rt::System::new().block_on(AccessToken::new("weapp", app_id, secret).get());
    println!("token=={:?}",rs);
    match rs {
        Some(s) => s.access_token,
        None => "".to_owned(),
    }
}

fn get_pub_template_keyword(access_token:&str,tid:u32) {
    let bll=SubTemplate::new(access_token);
    let rs = actix_rt::System::new().block_on(bll.get_pub_template_keyword(tid));

    println!("data={:?}",rs);
    
}
fn add_template(access_token:&str,tid:u32,kids:&Vec<u32>,desc:&str)->String {
    return "F8R6G-18D1ZDqzKw73IyoVCOGu2zBdUqbomX_XlDcWw".to_owned();
    let bll=SubTemplate::new(access_token);

    let rs = actix_rt::System::new().block_on(bll.add_template(tid,kids,desc));

    println!("data={:?}",rs);
    match rs {
        Ok(s) => s,
        Err(_) => "".to_owned(),
    }
}
fn send_sub_message(access_token:&str,open_id:&str,template_id:&str,template_data:&serde_json::Value,detail_url:&str) {
    let bll=Subscribe::new(access_token);
    let rs = actix_rt::System::new().block_on(bll.send_sub_message(open_id,template_id,template_data,detail_url));

    println!("data={:?}",rs);
    
}