use wanisabi::{client::Client, wrapper::assignments::AssignmentsFilter};

#[tokio::main]
async fn main() -> Result<(), wanisabi::Error> {
    let client = Client::default();
    let params = vec![
        //AssignmentsFilter::Burned(true),
        //AssignmentsFilter::Levels(vec![1]),
        AssignmentsFilter::ImmediatelyAvailableForLessons,
        //AssignmentsFilter::AvailableAfter(chrono::offset::Utc::now()),
        //AssignmentsFilter::Hidden(false),
    ];
    let assignments = client.get_assignments_filtered(params).await?;
    let d = assignments.data;
    let first = d.first().unwrap();
    let id = first.id;
    let assignment = client.get_assignment(id).await?;
    assert_eq!(assignment.data, first.data);
    Ok(())
}
