use ratelimit::Ratelimiter;
use reqwest::Response;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_with::EnumMap;
use std::{env, time::Duration};

pub struct Client {
    pub key: String,
    pub client: reqwest::Client,
    pub rate_limiter: Ratelimiter,
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize)]
pub struct QueryProcessor<T: DeserializeOwned + Serialize>(#[serde_as(as = "EnumMap")] pub Vec<T>);

pub mod macros {
    //Error parsing:
    //Wanikani can either return data, or an error.
    //So we need to first check whether the data we've got can be parsed into the struct we want
    //Otherwise parse it to an error
    //Otherwise return a parsing/network error
    #[macro_export]
    macro_rules! parse_error {
        ($return:ty, $res:ident, $self:ident) => {{
            if let Err(sleep) = $self.rate_limiter.try_wait() {
                tokio::time::sleep(sleep).await;
            }
            if let Ok(res) = serde_json::from_str::<$return>(&$res) {
                Ok(res)
            } else {
                Err(serde_json::from_str(&$res)?)
            }
        }};
    }
    #[macro_export]
    macro_rules! get {
        ($name:tt, $route:expr, $return:ty $(, $v:tt: $t:ty)*) => {
            pub async fn $name(&self $(, $v: $t)*) -> Result<$return, $crate::Error> {
                let url = String::from("https://api.wanikani.com/v2/") + &(format!($route));
                let req = self
                    .client
                    .get(url)
                    .bearer_auth(self.key.to_owned());
                let res = req.send().await?.text().await?;
                parse_error!($return, res, self)
            }
        };
        ($name:tt, $route:expr, $query:ty, $return:ty $(, $v:tt: $t:ty)*) => {
            pub async fn $name(&self, query: Vec<$query> $(, $v: $t)*) -> Result<$return, $crate::Error> {
                let qp = QueryProcessor(query);
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
                parse_error!($return, res, self)
            }
        };
    }

    #[macro_export]
    macro_rules! post {
        ($name:tt, $route:expr, $body:ty, $return:ty, $wrapper: ident, $attr: ident $(,$v:tt: $t:ty)*) => {
            pub async fn $name(&self, body: $body $(, $v: $t)*) -> Result<$return, $crate::Error> {
                let wrapped = $wrapper{
                    $attr: body
                };
                let req = self
                    .client
                    .post("https://api.wanikani.com/v2/".to_owned() + &(format!($route)))
                    .bearer_auth(self.key.to_owned())
                    .json(&wrapped);
                let res = req.send().await?.text().await?;
                parse_error!($return, res, self)
            }
        };
        ($name:tt, $route:expr, $body:ty, $return:ty $(,$v:tt: $t:ty)*) => {
            pub async fn $name(&self, body: &$body $(, $v: $t)*) -> Result<$return, $crate::Error> {
                let req = self
                    .client
                    .post("https://api.wanikani.com/v2/".to_owned() + &(format!($route)))
                    .bearer_auth(self.key.to_owned())
                    .json(body);
                let res = self.client.execute(req).await?.text().await?;
                parse_error!($return, res, self)
            }
        };
    }
    #[macro_export]
    macro_rules! put {
        ($name:tt, $route:expr, $body:ty, $return:ty, $wrapper:ident, $attr: ident $(,$v:tt: $t:ty)*) => {
            pub async fn $name(&self, body: $body $(, $v: $t)*) -> Result<$return, $crate::Error> {
                let wrapped = $wrapper{
                    $attr: body,
                };
                let req = self
                    .client
                    .put("https://api.wanikani.com/v2/".to_owned() + &(format!($route)))
                    .bearer_auth(self.key.to_owned())
                    .json(&wrapped);
                let res = req.send().await?.text().await?;
                parse_error!($return, res, self)
            }
        };
        ($name:tt, $route:expr, $body:ty, $return:ty $(,$v:tt: $t:ty)*) => {
            pub async fn $name(&self, body: &$body $(, $v: $t)*) -> Result<$return, $crate::Error> {
                let req = self
                    .client
                    .put("https://api.wanikani.com/v2/".to_owned() + &(format!($route)))
                    .bearer_auth(self.key.to_owned())
                    .json(body);
                let res = req.send().await?.text().await?;
                parse_error!($return, res, self)
            }
        };
    }
}

impl Client {
    pub fn new(key: String) -> Result<Self, crate::Error> {
        let client = reqwest::Client::new();
        let rate_limiter = Ratelimiter::builder(1, Duration::from_secs(60)).build()?;
        Ok(Self {
            key,
            client,
            rate_limiter,
        })
    }
    pub async fn get(&self, url: String) -> Result<Response, reqwest::Error> {
        Ok(self
            .client
            .get(url)
            .bearer_auth(self.key.clone())
            .send()
            .await?)
    }
}

impl Default for Client {
    fn default() -> Self {
        let client = reqwest::Client::new();
        let rate_limiter = Ratelimiter::builder(1, Duration::from_secs(1))
            .build()
            .unwrap();
        match env::var_os("WANIKANI_API_KEY") {
            Some(key) => Self {
                key: key.into_string().unwrap(),
                client,
                rate_limiter,
            },
            None => Self {
                key: String::new(),
                client,
                rate_limiter,
            },
        }
    }
}
