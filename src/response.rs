use crate::client::Client;
use chrono::{DateTime, Utc};
use paginate::Pages;
use serde::{Deserialize, Serialize};
use std::{
    any::type_name,
    fmt::{self, Display, Formatter},
    num::TryFromIntError,
};

///Any resource that doesn't have an ID. Namely user and summary
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UniqueResponse<T> {
    pub object: String,
    pub url: String,
    pub data_updated_at: DateTime<Utc>,
    pub data: T,
}

///Singular resource endpoints deliver information about a single entity, such as an assignment or subject.
///Any resource that has an ID
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IdResponse<T> {
    pub id: i64,
    pub object: String,
    pub url: String,
    pub data_updated_at: DateTime<Utc>,
    pub data: T,
}

///When there are more resources to return than the per-page limit, we use a cursor-based pagination scheme to move through the pages of results. We use the id of a resource as the cursor.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PagesResponse {
    ///The URL of the next page of results. If there are no more results, the value is null.
    pub next_url: Option<String>,
    ///The URL of the previous page of results. If there are no results at all or no previous page to go to, the value is null.
    pub previous_url: Option<String>,
    ///Maximum number of resources delivered for this collection.
    pub per_page: i64,
}

///Collections contain summary data about a bunch of resources, and also include each of the resources.
///By default, the maximum number of resources returned for collection endpoints is 500. Some endpoints may return a different size — reviews and subjects have a maximum size of 1,000.
///Any collection response has the per-page count in the pages.per_page attribute. Those same responses have a total_count attribute, too. That is a count of all resources available within the specified scope, not limited to pagination.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionResponse<T> {
    pub object: String,
    pub url: String,
    pub data_updated_at: Option<DateTime<Utc>>,
    pub pages: PagesResponse,
    pub total_count: i64,
    pub data: Vec<IdResponse<T>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Error {
    ///Something's gone wrong with the request before/while being sent
    Reqwest(String),
    ///The wrapper couldn't parse the result properly
    Deserialization(String),
    ///The ranges wanikani gave us don't fit
    RangeError(String),
    RateLimit(String),
    ///Wanikani has a problem with the data we've uploaded
    #[serde(untagged)]
    Wanikani {
        error: String,
        code: i64,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::Wanikani { error, code } => {
                write!(f, "Wanikani error: {} (Code {})", error, code)
            }
            Error::Reqwest(e) => {
                write!(f, "Reqwest error: {e}")
            }
            Error::Deserialization(e) => {
                write!(f, "Error deserializing response: {e}")
            }
            Error::RangeError(e) => {
                write!(f, "Range error: {e}")
            }
            Error::RateLimit(e) => {
                write!(f, "Rate Limit error: {e}")
            }
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Deserialization(value.to_string())
    }
}

impl From<TryFromIntError> for Error {
    fn from(value: TryFromIntError) -> Self {
        Self::RangeError(value.to_string())
    }
}

impl From<ratelimit::Error> for Error {
    fn from(value: ratelimit::Error) -> Self {
        Self::RateLimit(value.to_string())
    }
}

//TODO
impl<T: Deserialize<'static>> CollectionResponse<T> {
    pub async fn paginate(&mut self, c: Client) -> Result<Vec<IdResponse<T>>, crate::Error> {
        let mut res = vec![];
        //Bodge to find the endpoint of a given type;
        let type_name = type_name::<T>().to_lowercase();
        let endpoint = type_name.split("::").nth(2).unwrap().to_owned() + "s";
        let pages = Pages::new(
            self.total_count.try_into()?,
            self.pages.per_page.try_into()?,
        );
        for page in pages.into_iter() {
            let url = format!(
                "https://api.wanikani.com/{endpoint}/?page_after_id={}",
                page.offset
            );
            let a = c.get(url).await?.text().await?;
            res.push(a);
        }
        todo!()
    }
}
