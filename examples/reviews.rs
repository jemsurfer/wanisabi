use wanisabi::{
    client::Client,
    response::{CollectionResponse},
};
use wanisabi_model::review::Review;

#[tokio::main]
async fn main() -> Result<(), wanisabi::Error> {
    let client = Client::default();
    let params = vec![];
    //NOTE: this endpoint currently returns no data: (as of 06/02/24)
    //https://docs.api.wanikani.com/20170710/#get-all-reviews
    let reviews: CollectionResponse<Review> = client.get_reviews_filtered(params).await?;
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
