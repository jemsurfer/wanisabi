use wanikani_rs::{wanikani_client::WanikaniClient, wrapper::assignments::AssignmentsFilter};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = WanikaniClient::default();
    let params = vec![
        //AssignmentsFilter::Burned(true),
        //AssignmentsFilter::Levels(vec![1]),
        AssignmentsFilter::ImmediatelyAvailableForLessons(()),
        //AssignmentsFilter::AvailableAfter(chrono::offset::Utc::now()),
        //AssignmentsFilter::Hidden(false),
    ];

    let assignments = client.get_assignments_filtered(params).await;
    let d = assignments?.data;
    let first = d.first().unwrap();
    let id = first.clone().id;
    let assignment = client.get_assignment(id).await?;
    assert_eq!(assignment.data, first.data);
    Ok(())
}
