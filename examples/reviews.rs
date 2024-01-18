use wanikani_rs::{
    response::{CollectionResponse, ErrorResponse, ResourceResponse},
    wanikani_client::WanikaniClient,
};
use wanikani_rs_model::review::Review;

#[tokio::main]
async fn main() -> Result<(), ErrorResponse> {
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
