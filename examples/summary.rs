use wanikani_rs::wanikani_client::WanikaniClient;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = WanikaniClient::default();
    let resp = client.get_summary().await?;
    println!("{:#?}", resp);
    Ok(())
}
