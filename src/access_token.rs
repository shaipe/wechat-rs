//! copyright
//!

use crate::config::Config;

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessTokenBody {
    pub access_token: Option<String>,
    pub expires_in: Option<u32>,
    pub errcode: Option<u32>,
    pub errmsg: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessTokenValue {
    pub access_token: String,
    pub expires_in: u32,
}

pub struct AccessToken {
    pub access_token: Option<AccessTokenValue>,
    config: Config,
}

impl AccessToken {
    pub fn new(config: Config) -> Self {
        AccessToken {
            access_token: None,
            config,
        }
    }

    pub async fn update_token(&self) -> Result<AccessTokenValue> {
        let url = format!("https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid={}&secret={}", 
                          self.config.app_id, self.config.secret);
        let at = reqwest::get(&url).await?.json::<AccessTokenBody>().await?;
        let key = format!("accesstoken-{}", self.config.app_id);

        match at.access_token {
            Some(token) => {
                let atd = AccessTokenValue {
                    access_token: token,
                    expires_in: at.expires_in.unwrap(),
                };
                self.cache.set(&key, atd.clone()).await?;
                self.cache.ttl(&key, atd.expires_in).await?;
                Ok(atd)
            }
            None => Err(Error::AccessTokenError(at)),
        }
    }

    pub async fn get_token(&self) -> Result<AccessTokenValue> {
        let key = format!("accesstoken-{}", self.config.app_id);
        match self.cache.get(&key).await? {
            Some(token) => Ok(token),
            None => self.update_token().await,
        }
    }
}
