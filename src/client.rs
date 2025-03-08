use home::home_dir;
use ratelimit::Ratelimiter;
use reqwest::Response;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_with::EnumMap;
use sqlx::{migrate, migrate::MigrateDatabase};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::{env, time::Duration};

pub struct Client {
    pub key: String,
    pub client: reqwest::Client,
    pub rate_limiter: Option<Ratelimiter>,
    pub pool: Option<SqlitePool>,
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize)]
pub struct QueryProcessor<T: DeserializeOwned + Serialize>(#[serde_as(as = "EnumMap")] pub Vec<T>);

pub mod macros {
    //Before the query is executed, check for rate limits, and check cached data.
    #[macro_export]
    macro_rules! check_query {
        ($self:ident, $route:expr, $return:ty) => {
            if let Some(limiter) = &$self.rate_limiter {
                if let Err(sleep) = limiter.try_wait() {
                    tokio::time::sleep(sleep).await;
                }
            }
            if let Some(pool) = &$self.pool {
                //__METHOD__
                // - Check whether we've already got the result
                // - If we do, then tell the query to use the updated_after filter
                // - If the query returns nothing, then return the cached result
                // - If the query return something, then return the result of the query
                //and cache the result.
                //__

                //Stringify returns a quote-surrounded result
                let route = stringify!($route);
                //Extract the value inside the quotes, then remove any endpoint formatting (slashes)
                let table = route.split("\"").nth(1).unwrap().split("/").nth(0).unwrap();
                //Try and fetch any result which matches the query
                let res = sqlx::query_as!($return, format!("SELECT * FROM {table}"))
                    .fetch_one(&pool)
                    .await?;
            }
        };
    }
    //Error parsing:
    //Wanikani can either return data, or an error.
    //So we need to first check whether the data we've got can be parsed into the struct we want
    //Otherwise parse it to an error
    //Otherwise return a parsing/network error
    #[macro_export]
    macro_rules! process_response {
        ($return:ty, $res:ident) => {{
            if let Ok(res) = serde_json::from_str::<$return>(&$res) {
                Ok(res)
            } else {
                Err(serde_json::from_str(&$res)?)
            }
        }};
    }
    #[macro_export]
    macro_rules! get {
        //unfiltered (all) queries
        ($name:tt, $route:expr, $return:ty) => {
            pub async fn $name(&self) -> Result<$return, $crate::Error> {
                let url = String::from("https://api.wanikani.com/v2/") + &(format!($route));
                let req = self.client.get(url).bearer_auth(self.key.to_owned());
                let res = req.send().await?.text().await?;
                $crate::process_response!($return, res)
            }
        };
        //Single (id) queries
        ($name:tt, $route:expr, $return:ty, $id: tt: $t: ty) => {
            pub async fn $name(&self, $id: $t) -> Result<$return, $crate::Error> {
                let url = String::from("https://api.wanikani.com/v2/") + &(format!($route));
                let req = self.client.get(url).bearer_auth(self.key.to_owned());
                let res = req.send().await?.text().await?;
                $crate::process_response!($return, res)
            }
        };
        //Filtered queries
        ($name:tt, $route:expr, $query:ty, $return:ty) => {
            pub async fn $name(&self, query: Vec<$query>) -> Result<$return, $crate::Error> {
                let qp = $crate::client::QueryProcessor(query);
                let re = regex::Regex::new(r"\[\d+\]").unwrap();
                let qs = qs::to_string(&qp).unwrap();
                let qs = re.replace_all(qs.as_str(), "");
                let mut q_map: std::collections::HashMap<String, String> =
                    std::collections::HashMap::new();
                for q in qs.split("&") {
                    let mut item = q.split("=");
                    let k = item.next().unwrap();
                    let v = item.next().unwrap_or("");
                    if q_map.contains_key(&k.to_string()) {
                        q_map.insert(
                            k.to_string(),
                            format!("{},{}", q_map.get(&k.to_string()).unwrap(), v),
                        );
                    } else {
                        q_map.insert(k.to_string(), v.to_string());
                    }
                }
                let mut queries: Vec<String> = vec![];
                for (k, v) in q_map {
                    queries.push(format!("{}={}", k, v));
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
                $crate::process_response!($return, res)
            }
        };
    }
    #[macro_export]
    macro_rules! post {
        //Post requests need to be wrapped in a field matching the endpoint name.
        //E.g.
        //{
        //  assignments: data
        //}
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
                $crate::process_response!($return, res)
            }
        };
    }
    #[macro_export]
    macro_rules! put {
        //Most put requests need to be wrapped the same as post
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
                $crate::process_response!($return, res)
            }
        };
        //Apart from assignments
        ($name:tt, $route:expr, $body:ty, $return:ty $(,$v:tt: $t:ty)*) => {
            pub async fn $name(&self, body: &$body $(, $v: $t)*) -> Result<$return, $crate::Error> {
                let req = self
                    .client
                    .put("https://api.wanikani.com/v2/".to_owned() + &(format!($route)))
                    .bearer_auth(self.key.to_owned())
                    .json(body);
                let res = req.send().await?.text().await?;
                $crate::process_response!($return, res)
            }
        };
    }
}

impl Client {
    //The default trait cannot be derived for async functions, so it's in the main impl block
    //By default, enable rate limiting & caching
    pub async fn default() -> Result<Self, crate::Error> {
        let client = reqwest::Client::new();
        let (rate_limiter, pool) = rate_limiter_and_pool(true, true).await?;
        match env::var_os("WANIKANI_API_KEY") {
            Some(key) => Ok(Self {
                key: key.into_string().unwrap(),
                client,
                rate_limiter,
                pool,
            }),
            None => Err(crate::Error::NoApiKey),
        }
    }

    pub async fn new(key: String, rate_limit: bool, cache: bool) -> Result<Self, crate::Error> {
        let client = reqwest::Client::new();
        let (rate_limiter, pool) = rate_limiter_and_pool(rate_limit, cache).await?;
        Ok(Self {
            key,
            client,
            rate_limiter,
            pool,
        })
    }

    //Generic get function for other URLs
    pub async fn get(&self, url: String) -> Result<Response, reqwest::Error> {
        if let Some(limiter) = &self.rate_limiter {
            if let Err(sleep) = limiter.try_wait() {
                tokio::time::sleep(sleep).await;
            }
        }
        self.client
            .get(url)
            .bearer_auth(self.key.clone())
            .send()
            .await
    }
}

async fn rate_limiter_and_pool(
    limit: bool,
    pool: bool,
) -> Result<(Option<Ratelimiter>, Option<SqlitePool>), crate::Error> {
    let rate_limiter = if limit {
        Some(
            Ratelimiter::builder(60, Duration::from_secs(60))
                .max_tokens(60)
                .initial_available(60)
                .build()?,
        )
    } else {
        None
    };
    let db_pool = if pool {
        let dir = home_dir()
            .map(|mut path| {
                path.push(".wanisabi.db");
                path.to_str().unwrap_or_default().to_owned()
            })
            .unwrap_or_default();

        //Create the database if it doesn't exist
        if !sqlx::Sqlite::database_exists(&dir).await? {
            sqlx::Sqlite::create_database(&dir).await?;
        }
        let pool = SqlitePoolOptions::new().connect(&dir).await?;
        //Run migrations in case this is the first time the database has been used
        migrate!("./migrations").run(&pool).await?;
        Some(pool)
    } else {
        None
    };
    Ok((rate_limiter, db_pool))
}
