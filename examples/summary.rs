use wanisabi::client::Client;

#[tokio::main]
async fn main() -> Result<(), wanisabi::Error> {
    let client = Client::default().await?;
    let resp = client.get_summary().await?;
    println!("{:#?}", resp);
    Ok(())
}
