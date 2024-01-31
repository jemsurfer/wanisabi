use wanisabi::{
    response::{CollectionResponse, Error, ResourceResponse},
    wanikani_client::WanikaniClient,
};
use wanisabi_model::review::Review;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = WanikaniClient::default();
    let params = vec![];

    let reviews: CollectionResponse<ResourceResponse<Review>> =
        client.get_reviews_filtered(params).await?;
    let d = reviews.data;
    if d.is_empty() {
        return Ok(());
    }
    let first = d.first().unwrap();
    let id = first.id;
    let review = client.get_review(id).await?;
    assert_eq!(review.data, first.data);
    Ok(())
}
