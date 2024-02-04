use wanisabi::{client::Client, response::UserResponse};

#[tokio::main]
async fn main() -> Result<(), wanisabi::Error> {
    let client = Client::default();
    let resp: UserResponse = client.get_user_info().await?;
    println!("{:#?}", resp);
    Ok(())
}
