//! copyright © shaipe 2020 - persent

use crate::config::Config;
use reqwest::Client as HttpClient;
use std::time::Duration;

/// 
pub(crate) struct Client {

    pub(crate) config: Config,

    pub(crate) client: HttpClient

}

impl Client {
    pub fn new(config: Config) -> Self {
        Client {
            config: config,
            client: HttpClient::builder()
                .timeout(Duration::from_secs(5))
                .connect_timeout(Duration::from_secs(5))
                .build()
                .unwrap(),
        }
    }
}