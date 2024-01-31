use wanisabi::{response::Error, wanikani_client::WanikaniClient};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = WanikaniClient::default();
    let resp = client.get_summary().await?;
    println!("{:#?}", resp);
    Ok(())
}
