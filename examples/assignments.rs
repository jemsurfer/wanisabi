use wanikani_rs::{response::CollectionResponse, wanikani_client::WanikaniClient};
use wanikani_rs_model::assignment::Assignment;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = WanikaniClient::default();
    let resp = client.get_assignments().await;
    println!("{:#?}", resp);
    Ok(())
}
