use wanikani_rs::{
    response::{CollectionResponse, ResourceResponse},
    wanikani_client::WanikaniClient,
};
use wanikani_rs_model::voice_actor::VoiceActor;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = WanikaniClient::default();
    let params = vec![];

    let voice_actors: CollectionResponse<ResourceResponse<VoiceActor>> =
        client.get_voice_actors_filtered(params).await?;
    let d = voice_actors.data;
    if d.is_empty() {
        return Ok(());
    }
    let first = d.first().unwrap();
    let id = first.id;
    let voice_actor = client.get_voice_actor(id).await?;
    assert_eq!(voice_actor.data, first.data);
    Ok(())
}
