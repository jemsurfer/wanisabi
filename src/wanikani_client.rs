use reqwest::Result;
use serde::{de, Serialize};
use std::env;

use crate::model::assignment::Assignment;
use crate::response::{CollectionResponse, ResourceResponse, UserResponse};
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct WanikaniClient {
    pub key: String,
    pub client: reqwest::Client,
}

pub mod macros {
    #[macro_export]
    macro_rules! get {
        ($name:tt, $route:expr, $query:ty, $return:ty) => {
            pub async fn $name(&self, query: &Vec<($query, &str)>) -> Result<$return, Error> {
                let req = self
                    .client
                    .get(format!("https://api.wanikani.com/v2/{}", $route))
                    .bearer_auth(self.key.to_owned())
                    .query(query);
                req.send().await?.json().await
            }
        };
        ($name:tt, $route:expr, $return:ty) => {
            pub async fn $name(&self) -> Result<$return, Error> {
                let req = self
                    .client
                    .get(format!("https://api.wanikani.com/v2/{}", $route))
                    .bearer_auth(self.key.to_owned());
                req.send().await?.json().await
            }
        };
    }

    #[macro_export]
    macro_rules! post {
        ($name:tt, $route:expr, $body:ty, $return:ty) => {
            pub async fn $name(&self, body: $body) -> Result<$return, Error> {
                let req = self
                    .client
                    .post("https://api.wanikani.com/v2/".to_owned() + $route)
                    .bearer_auth(self.key.to_owned());
                req.send().await?.json().await
            }
        };
    }
}
impl WanikaniClient {
    pub fn new(key: String) -> Self {
        let client = reqwest::Client::new();
        Self { key, client }
    }
    // async fn _get<T, Q>(&self, url: &str, query: &Q) -> Result<T>
    // where
    //     T: de::DeserializeOwned + Debug,
    //     Q: Serialize + ?Sized,
    // {
    //     let req = self
    //         .client
    //         .get("https://api.wanikani.com/v2".to_string() + url)
    //         .bearer_auth(self.key.to_owned())
    //         .query(query);
    //     req.send().await?.json().await
    // }

    // pub async fn get_assignments(
    //     &self,
    // ) -> Result<CollectionResponse<ResourceResponse<Assignment>>> {
    //     GET!(self, "/assignments")
    // }

    // pub async fn get_assignment(&self, id: i32) -> Result<ResourceResponse<Assignment>> {
    //     GET!(self, format!("/assignments/{id}").as_str())
    // }
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
