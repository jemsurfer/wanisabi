use std::env;

use reqwest::Result;
use serde::de;

use crate::model::assignment::Assignment;
use crate::response::{CollectionResponse, ResourceResponse, UserResponse};
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct WanikaniClient {
    pub key: String,
    client: reqwest::Client,
}

macro_rules! GET {
    ($self:ident, $route:expr) => {
        $self._get($route).await
    };
}

impl WanikaniClient {
    pub fn new(key: String) -> Self {
        let client = reqwest::Client::new();
        Self { key, client }
    }
    async fn _get<T>(&self, url: &str) -> Result<T>
    where
        T: de::DeserializeOwned + Debug,
    {
        self.client
            .get("https://api.wanikani.com/v2".to_string() + url)
            .bearer_auth(self.key.to_owned())
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_user_info(&self) -> Result<UserResponse> {
        GET!(self, "/user")
    }

    pub async fn get_assignments(
        &self,
    ) -> Result<CollectionResponse<ResourceResponse<Assignment>>> {
        GET!(self, "/assignments")
    }

    pub async fn get_assignment(&self, id: i32) -> Result<ResourceResponse<Assignment>> {
        GET!(self, format!("/assignments/{id}").as_str())
    }
}

impl Default for WanikaniClient {
    fn default() -> Self {
        let client = reqwest::Client::new();
        match env::var_os("WANIKANI_API_KEY") {
            Some(key) => Self {
                key: key.into_string().unwrap(),
                client,
            },
            None => Self {
                key: String::new(),
                client,
            },
        }
    }
}
