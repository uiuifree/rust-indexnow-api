use crate::error::GoogleApiError;
use serde_json::json;
use std::fmt::Debug;

#[derive(Default, Debug)]
pub(crate) struct HttpClient {}

impl HttpClient {

    pub async fn post<U>(url: &str, params: U) -> Result<(), GoogleApiError>
    where
        U: serde::Serialize + std::fmt::Debug,
    {
        let response = reqwest::Client::new()
            .post(format!("{}", url))
            .header("Host", "api.indexnow.org");
        let response = response.json(&json!(params)).send().await;
        if let Err(err) = response {
            return Err(GoogleApiError::Connection(err.to_string()));
        }
        let response = response.unwrap();
        let status = response.status();
        let value = response.text().await;
        if status != 200 {
            return Err(GoogleApiError::Connection(value.unwrap().to_string()));
        }

        Ok(())
    }
}
