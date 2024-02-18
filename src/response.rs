use crate::client::Client;
use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
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

impl<T: DeserializeOwned + Clone> CollectionResponse<T> {
    pub async fn paginate(&self, c: Client) -> Result<Vec<IdResponse<T>>, crate::Error> {
        let mut responses = self.data.to_vec();
        if self.pages.next_url.is_none() {
            return Ok(responses);
        }
        let mut next = self.pages.next_url.clone();
        while let Some(url) = next {
            let a: CollectionResponse<T> = serde_json::from_str(&c.get(url).await?.text().await?)?;
            responses.extend(a.data);
            next = a.pages.next_url;
        }
        Ok(responses)
    }
}
