use wanikani_rs::{
    response::{CollectionResponse, ErrorResponse, ResourceResponse},
    wanikani_client::WanikaniClient,
};
use wanikani_rs_model::review_statistic::ReviewStatistic;

#[tokio::main]
async fn main() -> Result<(), ErrorResponse> {
    let client = WanikaniClient::default();
    let params = vec![];

    let review_statistics: CollectionResponse<ResourceResponse<ReviewStatistic>> =
        client.get_review_statistics_filtered(params).await?;
    let d = review_statistics.data;
    if d.is_empty() {
        return Ok(());
    }
    let first = d.first().unwrap();
    let id = first.id;
    let review_statistic = client.get_review_statistic(id).await?;
    assert_eq!(review_statistic.data, first.data);
    Ok(())
}
