use wanikani_rs::response::UserResponse;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let resp: UserResponse = client
        .get("https://api.wanikani.com/v2/user")
        .bearer_auth("a6453799-9601-483b-8976-492e87b1c46a").send().await?.json().await?;
    println!("{:#?}", resp);
    Ok(())
}
