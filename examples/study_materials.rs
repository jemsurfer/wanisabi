use wanisabi::{client::Client, response::CollectionResponse};
use wanisabi_model::study_material::StudyMaterial;

#[tokio::main]
async fn main() -> Result<(), wanisabi::Error> {
    let client = Client::default().await?;
    let params = vec![];
    let study_materials: CollectionResponse<StudyMaterial> =
        client.get_study_materials_filtered(params).await?;
    let d = study_materials.data;
    if d.is_empty() {
        return Ok(());
    }
    let first = d.first().unwrap();
    let id = first.id;
    let study_material = client.get_study_material(id).await?;
    assert_eq!(study_material.data, first.data);
    Ok(())
}
