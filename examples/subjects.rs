use wanisabi::{
    response::{CollectionResponse, Error, ResourceResponse},
    wanikani_client::WanikaniClient,
};
use wanisabi_model::subject::Subject;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = WanikaniClient::default();
    let params = vec![];

    let subjects: CollectionResponse<ResourceResponse<Subject>> =
        client.get_subjects_filtered(params).await?;
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
