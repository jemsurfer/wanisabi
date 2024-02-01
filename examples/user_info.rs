use wanisabi::{
    response::{UserResponse},
    wanikani_client::WanikaniClient,
};

#[tokio::main]
async fn main() -> Result<(), wanisabi::Error> {
    let client = WanikaniClient::default();
    let resp: UserResponse = client.get_user_info().await?;
    println!("{:#?}", resp);
    Ok(())
}
