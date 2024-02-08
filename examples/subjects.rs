use wanisabi::{
    client::Client,
    response::{CollectionResponse},
};
use wanisabi_model::subject::Subject;

#[tokio::main]
async fn main() -> Result<(), wanisabi::Error> {
    let client = Client::default();
    let params = vec![];
    let subjects: CollectionResponse<Subject> = client.get_subjects_filtered(params).await?;
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
