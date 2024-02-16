use wanisabi::{client::Client, response::CollectionResponse};
use wanisabi_model::voice_actor::VoiceActor;

#[tokio::main]
async fn main() -> Result<(), wanisabi::Error> {
    let client = Client::default().await?;
    let params = vec![];

    let voice_actors: CollectionResponse<VoiceActor> =
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
