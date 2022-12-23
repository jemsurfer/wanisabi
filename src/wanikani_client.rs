use std::env;

use serde::{Deserialize, de};

use crate::response::UserResponse;

#[derive(Clone, Debug)]
struct WanikaniClient {
    pub key: String,
    client: reqwest::Client
}

impl WanikaniClient {
    pub fn new(key: String) -> Self {
        let client = reqwest::Client::new();
        Self { key, client }
    }
    async fn _get<T: de::DeserializeOwned>(&self, url: &str) -> Result<T, reqwest::Error> {
        self.client.get("https://api.wanikani.com/v2".to_string() + url).bearer_auth(self.key.to_owned()).send().await?.json().await
    }

    pub async fn get_user_info(&self) -> Result<UserResponse, reqwest::Error> {
        self._get::<UserResponse>("/user").await
    }
    // add code here
}

impl Default for WanikaniClient {
    fn default() -> Self {
        let client = reqwest::Client::new();
        match env::var_os("WANIKANI_API_KEY") {
            Some(key) => Self { key: key.into_string().unwrap(), client },
            None => {
                Self { key: String::new(), client }
            }
        }
    }


}
