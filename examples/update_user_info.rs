use wanisabi::{
    client::Client,
    wrapper::user::{
        LessonPresentationOrder, ReviewsPresentationOrder, UserPreferencesUpdate, UserUpdate,
    },
};

#[tokio::main]
async fn main() -> Result<(), wanisabi::Error> {
    let client = Client::default().await?;
    let body = UserUpdate {
        preferences: UserPreferencesUpdate {
            default_voice_actor_id: None,
            lessons_autoplay_audio: Some(true),
            lessons_batch_size: Some(5),
            lessons_presentation_order: Some(LessonPresentationOrder::default()),
            reviews_autoplay_audio: Some(true),
            reviews_display_srs_indicator: Some(true),
            reviews_presentation_order: Some(ReviewsPresentationOrder::default()),
        },
    };
    client.update_user_info(body).await?;
    Ok(())
}
