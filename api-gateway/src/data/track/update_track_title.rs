use schema::{
    courses::{courses_client::CoursesClient, UpdateTrackTitleRequest},
    shared::User,
};

use crate::{entities::Track, errors::GatewayError};

pub async fn update_track_title(
    active_user: Option<User>,
    track_id: i32,
    title: String,
    channel: tonic::transport::Channel,
) -> Result<Track, GatewayError> {
    let mut client = CoursesClient::new(channel);
    let request = tonic::Request::new(UpdateTrackTitleRequest {
        active_user,
        track_id,
        title,
    });
    let response = client.update_track_title(request).await?.into_inner();
    let track = response.track.unwrap();
    Ok(Track {
        id: track.id,
        track_number: track.track_number,
        audio_path: track.audio_path,
        title: track.title,
        document_id: track.document_id,
    })
}
