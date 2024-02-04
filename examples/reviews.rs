use wanisabi::{
    response::{CollectionResponse, ResourceResponse},
    client::Client,
};
use wanisabi_model::review::Review;

#[tokio::main]
async fn main() -> Result<(), wanisabi::Error> {
    let client = Client::default();
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
