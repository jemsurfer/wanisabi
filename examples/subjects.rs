use wanisabi::client::Client;

#[tokio::main]
async fn main() -> Result<(), wanisabi::Error> {
    let client = Client::default().await?;
    let subjects = client.get_subjects().await?;
    dbg!(&subjects);
    let d = subjects.data;
    if d.is_empty() {
        return Ok(());
    }
    let first = d.first().unwrap();
    let id = first.id;
    let subject = client.get_subject(id).await?;
    dbg!(subject);
    Ok(())
}
