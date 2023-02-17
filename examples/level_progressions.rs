use wanikani_rs::{
    response::{CollectionResponse, ResourceResponse},
    wanikani_client::WanikaniClient,
    wrapper::levels::LevelProgressionFilter,
};
use wanikani_rs_model::level_progression::LevelProgression;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = WanikaniClient::default();
    let params = vec![LevelProgressionFilter::Ids(vec![1, 2])];

    let assignments: CollectionResponse<ResourceResponse<LevelProgression>> =
        client.get_level_progressions_filtered(params).await?;
    let d = assignments.data;
    let first = d.first().unwrap();
    let id = first.id;
    let assignment = client.get_level_progression(id).await?;
    assert_eq!(assignment.data, first.data);
    Ok(())
}
