use wanikani_rs::{response::ErrorResponse, wanikani_client::WanikaniClient};

#[tokio::main]
async fn main() -> Result<(), ErrorResponse> {
    let client = WanikaniClient::default();
    let resp = client.get_summary().await?;
    println!("{:#?}", resp);
    Ok(())
}
