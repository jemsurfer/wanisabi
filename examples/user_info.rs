use wanisabi::{
    response::{Error, UserResponse},
    wanikani_client::WanikaniClient,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = WanikaniClient::default();
    let resp: UserResponse = client.get_user_info().await?;
    println!("{:#?}", resp);
    Ok(())
}
