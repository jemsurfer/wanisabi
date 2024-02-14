use wanisabi::{client::Client, response::CollectionResponse};
use wanisabi_model::review_statistic::ReviewStatistic;

#[tokio::main]
async fn main() -> Result<(), wanisabi::Error> {
    let client = Client::default();
    let params = vec![];

    let review_statistics: CollectionResponse<ReviewStatistic> =
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
