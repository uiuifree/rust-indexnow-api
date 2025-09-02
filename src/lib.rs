mod error;
mod http;

use crate::http::HttpClient;
use crate::GoogleApiError;
pub use error::*;
use serde::{Deserialize, Serialize};

/// API Access Endpoint
pub struct IndexNowApi {
    search_engine: String,
    host: String,
    key: String,
    key_location: Option<String>,
}

impl IndexNowApi {
    pub fn new<T: ToString, U: ToString>(host: T, key: U) -> IndexNowApi {
        IndexNowApi {
            search_engine: "https://api.indexnow.org".to_string(),
            host: host.to_string(),
            key: key.to_string(),
            key_location: None,
        }
    }
    pub fn set_search_engine<T: ToString>(&mut self, search_engine: T) {
        self.search_engine = search_engine.to_string()
    }
    pub fn set_key_location<T: ToString>(&mut self, key_location: T) {
        self.key_location = Some(key_location.to_string())
    }


    pub async fn send_urls<T:ToString>(&self, urls: Vec<T>) -> Result<(), GoogleApiError> {
        HttpClient::post(
            format!("{}/IndexNow", self.search_engine).as_str(),
            SendData {
                url_list: urls.iter().map(|q|q.to_string()).collect(),
                host: self.host.to_string(),
                key: self.key.to_string(),
                key_location: self.key_location.clone(),
            },
        )
        .await
    }

}

/// URL Notification Type
/// UPDATE or DELETE
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct SendData {
    #[serde(rename = "urlList")]
    url_list: Vec<String>,
    host: String,
    key: String,
    #[serde(rename = "keyLocation")]
    key_location: Option<String>,
}
