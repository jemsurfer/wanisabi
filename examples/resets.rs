use wanisabi::{client::Client, response::CollectionResponse};
use wanisabi_model::reset::Reset;

#[tokio::main]
async fn main() -> Result<(), wanisabi::Error> {
    let client = Client::default().await?;
    let params = vec![];

    let resets: CollectionResponse<Reset> = client.get_resets_filtered(params).await?;
    let d = resets.data;
    if d.is_empty() {
        return Ok(());
    }
    let first = d.first().unwrap();
    let id = first.id;
    let reset = client.get_reset(id).await?;
    assert_eq!(reset.data, first.data);
    Ok(())
}
