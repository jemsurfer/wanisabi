use wanisabi::{
    response::{CollectionResponse, Error, ResourceResponse},
    wanikani_client::WanikaniClient,
};
use wanisabi_model::spaced_repetition_system::SpacedRepetitionSystem;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = WanikaniClient::default();
    let params = vec![];

    let spaced_repetition_systems: CollectionResponse<ResourceResponse<SpacedRepetitionSystem>> =
        client
            .get_spaced_repetition_systems_filtered(params)
            .await?;
    let d = spaced_repetition_systems.data;
    if d.is_empty() {
        return Ok(());
    }
    let first = d.first().unwrap();
    let id = first.id;
    let spaced_repetition_system = client.get_spaced_repetition_system(id).await?;
    assert_eq!(spaced_repetition_system.data, first.data);
    Ok(())
}
