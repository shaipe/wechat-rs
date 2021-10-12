//! copyright © ecdata.cn 2021 - present
//! 接口凭证管理，直接使用文件进行管理
//! created by shaipe 20211012

/// Access Token对象
#[derive(Debug, Clone)] 
pub struct AccessToken {
    // 应用类型
    pub app_type: String,
    // 访问token
    pub access_token: String,
    // access_token获取时间
    pub create_time: u64,
    // 有效期
    pub expires: u64,
}

impl AccessToken {
    pub fn load(file_path: &str) -> Self {
        AccessToken {
            app_type: "weapp".to_owned(),
            access_token: "".to_owned(),
            create_time: 0,
            expires: 0,
        }
    }
}
