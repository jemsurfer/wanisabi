use wanisabi::client::Client;

#[tokio::main]
async fn main() -> Result<(), wanisabi::Error> {
    let client = Client::default();
    let resp = client.get_user_info().await?;
    println!("{:#?}", resp);
    Ok(())
}
