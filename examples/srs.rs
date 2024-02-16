use wanisabi::{client::Client, response::CollectionResponse};
use wanisabi_model::spaced_repetition_system::SpacedRepetitionSystem;

#[tokio::main]
async fn main() -> Result<(), wanisabi::Error> {
    let client = Client::default().await?;
    let params = vec![];

    let spaced_repetition_systems: CollectionResponse<SpacedRepetitionSystem> = client
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
