use wanisabi::{
    client::Client, response::CollectionResponse,
    wrapper::level_progressions::LevelProgressionFilter,
};
use wanisabi_model::level_progression::LevelProgression;

#[tokio::main]
async fn main() -> Result<(), wanisabi::Error> {
    let client = Client::default().await?;
    let params = vec![LevelProgressionFilter::Ids(vec![1, 2])];
    let lps: CollectionResponse<LevelProgression> =
        client.get_level_progressions_filtered(params).await?;
    let d = lps.data;
    if d.is_empty() {
        return Ok(());
    }
    let first = d.first().unwrap();
    let id = first.id;
    let lp = client.get_level_progression(id).await?;
    assert_eq!(lp.data, first.data);
    Ok(())
}
