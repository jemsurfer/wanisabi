use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_with::EnumMap;
use std::env;
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
    //Error parsing:
    //Wanikani can either return data, or an error.
    //So we need to first check whether the data we've got can be parsed into the struct we want
    //Otherwise parse it to an error
    //Otherwise return a parsing error
    #[macro_export]
    macro_rules! get {
        ($name:tt, $route:expr, $return:ty $(, $v:tt: $t:ty)*) => {
            pub async fn $name(&self $(, $v: $t)*) -> Result<$return, ErrorResponse> {
                let url = String::from("https://api.wanikani.com/v2/") + &(format!($route));
                let req = self
                    .client
                    .get(url)
                    .bearer_auth(self.key.to_owned());
                let res = req.send().await?.text().await?;
                if let Ok(res) = serde_json::from_str::<$return>(&res){
                    return Ok(res);
                } else {
                    return Err(ErrorResponse::WanikaniError(serde_json::from_str::<WanikaniError>(&res)?));
                }
            }
        };
        ($name:tt, $route:expr, $query:ty, $return:ty $(, $v:tt: $t:ty)*) => {
            pub async fn $name(&self, query: Vec<$query> $(, $v: $t)*) -> Result<$return, ErrorResponse> {
                let qp: QP<$query> = QP(query);
                let re = regex::Regex::new(r"\[\d+\]").unwrap();
                let qs = qs::to_string(&qp).unwrap();
                let qs = re.replace_all(qs.as_str(), "");
                let mut q_map: std::collections::HashMap<String, String> = std::collections::HashMap::new();
                for q in qs.split("&") {
                    let mut item = q.split("=");
                    let k = item.next().unwrap();
                    let v = item.next().unwrap_or("");

                    if q_map.contains_key(&k.to_string()) {
                        q_map.insert(k.to_string(), format!("{},{}", q_map.get(&k.to_string()).unwrap(), v));
                    } else {
                        q_map.insert(k.to_string(),v.to_string());
                    }
                }
                let mut queries: Vec<String> = vec![];
                for (k,v) in q_map {
                    queries.push(format!("{}={}",k,v));
                }
                let qs = queries.join("&");

                let url = String::from("https://api.wanikani.com/v2/") + &(format!($route));
                let mut req = self
                    .client
                    .get(url)
                    .bearer_auth(self.key.to_owned())
                    .build()?;
                req.url_mut().set_query(Some(&qs));
                let res = self.client.execute(req).await?.text().await?;
                if let Ok(res) = serde_json::from_str::<$return>(&res){
                    return Ok(res);
                } else {
                    return Err(ErrorResponse::WanikaniError(serde_json::from_str::<WanikaniError>(&res)?));
                }
            }
        };
    }

    #[macro_export]
    macro_rules! post {
        ($name:tt, $route:expr, $body:ty, $return:ty, $wrapper: ident, $attr: ident $(,$v:tt: $t:ty)*) => {
            pub async fn $name(&self, body: $body $(, $v: $t)*) -> Result<$return, ErrorResponse> {
                let wrapped = $wrapper{
                    $attr: body
                };
                let req = self
                    .client
                    .post("https://api.wanikani.com/v2/".to_owned() + &(format!($route)))
                    .bearer_auth(self.key.to_owned())
                    .json(&wrapped);
                let res = req.send().await?.text().await?;
                if let Ok(res) = serde_json::from_str::<$return>(&res){
                    return Ok(res);
                } else {
                    return Err(ErrorResponse::WanikaniError(serde_json::from_str::<WanikaniError>(&res)?));
                }

            }
        };
        ($name:tt, $route:expr, $body:ty, $return:ty $(,$v:tt: $t:ty)*) => {
            pub async fn $name(&self, body: &$body $(, $v: $t)*) -> Result<$return, ErrorResponse> {
                let req = self
                    .client
                    .post("https://api.wanikani.com/v2/".to_owned() + &(format!($route)))
                    .bearer_auth(self.key.to_owned())
                    .json(body);
                let res = self.client.execute(req).await?.text().await?;
                if let Ok(res) = serde_json::from_str::<$return>(&res){
                    return Ok(res);
                } else {
                    return Err(ErrorResponse::WanikaniError(serde_json::from_str::<WanikaniError>(&res)?));
                }
            }
        };
    }
    #[macro_export]
    macro_rules! put {
        ($name:tt, $route:expr, $body:ty, $return:ty, $wrapper:ident, $attr: ident $(,$v:tt: $t:ty)*) => {
            pub async fn $name(&self, body: $body $(, $v: $t)*) -> Result<$return, ErrorResponse> {
                let wrapped = $wrapper{
                    $attr: body,
                };
                let req = self
                    .client
                    .put("https://api.wanikani.com/v2/".to_owned() + &(format!($route)))
                    .bearer_auth(self.key.to_owned())
                    .json(&wrapped);
                let res = req.send().await?.text().await?;
                if let Ok(res) = serde_json::from_str::<$return>(&res){
                    return Ok(res);
                } else {
                    return Err(ErrorResponse::WanikaniError(serde_json::from_str::<WanikaniError>(&res)?));
                }
            }
        };
        ($name:tt, $route:expr, $body:ty, $return:ty $(,$v:tt: $t:ty)*) => {
            pub async fn $name(&self, body: &$body $(, $v: $t)*) -> Result<$return, ErrorResponse> {
                let req = self
                    .client
                    .put("https://api.wanikani.com/v2/".to_owned() + &(format!($route)))
                    .bearer_auth(self.key.to_owned())
                    .json(body);
                let res = req.send().await?.text().await?;
                if let Ok(res) = serde_json::from_str::<$return>(&res){
                    return Ok(res);
                } else {
                    return Err(ErrorResponse::WanikaniError(serde_json::from_str::<WanikaniError>(&res)?));
                }
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
