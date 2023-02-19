use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::env;

use serde_with::EnumMap;
use std::fmt::Debug;
#[derive(Clone, Debug)]
pub struct WanikaniClient {
    pub key: String,
    pub client: reqwest::Client,
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize)]
pub struct QP<T: DeserializeOwned + Serialize>(#[serde_as(as = "EnumMap")] pub Vec<T>);

pub mod macros {
    #[macro_export]
    macro_rules! get {
        ($name:tt, $route:expr, $return:ty $(, $v:tt: $t:ty)*) => {
            pub async fn $name(&self $(, $v: $t)*) -> Result<$return, Error> {
                let url = String::from("https://api.wanikani.com/v2/") + &(format!($route));
                let req = self
                    .client
                    .get(url)
                    .bearer_auth(self.key.to_owned());
                req.send().await?.json().await
            }
        };
        ($name:tt, $route:expr, $query:ty, $return:ty $(, $v:tt: $t:ty)*) => {
            pub async fn $name(&self, query: Vec<$query> $(, $v: $t)*) -> Result<$return, Error> {
                let qp: QP<$query> = QP(query);
                let qs = qs::to_string(&qp).unwrap();
                let url = String::from("https://api.wanikani.com/v2/") + &(format!($route));
                let mut req = self
                    .client
                    .get(url)
                    .bearer_auth(self.key.to_owned())
                    .build()?;
                req.url_mut().set_query(Some(&qs));
                self.client.execute(req).await?.json().await
            }
        };
    }

    #[macro_export]
    macro_rules! post {
        ($name:tt, $route:expr, $body:ty, $return:ty $(,$v:tt: $t:ty)*) => {
            pub async fn $name(&self, body: &$body $(, $v: $t)*) -> Result<$return, Error> {
                let req = self
                    .client
                    .post("https://api.wanikani.com/v2/".to_owned() + &(format!($route)))
                    .bearer_auth(self.key.to_owned())
                    .json(body);
                req.send().await?.json().await
            }
        };
    }

    #[macro_export]
    macro_rules! put {
        ($name:tt, $route:expr, $body:ty, $return:ty $(,$v:tt: $t:ty)*) => {
            pub async fn $name(&self, body: &$body $(, $v: $t)*) -> Result<$return, Error> {
                let req = self
                    .client
                    .put("https://api.wanikani.com/v2/".to_owned() + &(format!($route)))
                    .bearer_auth(self.key.to_owned())
                    .json(body);
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

    // pub async fn get_assignment(&self, id: i64) -> Result<ResourceResponse<Assignment>> {
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
