use wanikani_rs::{
    response::{ErrorResponse, UserResponse},
    wanikani_client::WanikaniClient,
};

#[tokio::main]
async fn main() -> Result<(), ErrorResponse> {
    let client = WanikaniClient::default();
    let resp: UserResponse = client.get_user_info().await?;
    println!("{:#?}", resp);
    Ok(())
}
