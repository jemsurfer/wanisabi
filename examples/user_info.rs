use wanikani_rs::{response::UserResponse, wanikani_client::WanikaniClient};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = WanikaniClient::default();
    let resp: UserResponse = client.get_user_info().await?;
    println!("{:#?}", resp);
    Ok(())
}
