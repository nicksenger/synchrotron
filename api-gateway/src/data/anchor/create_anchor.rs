use schema::{
    courses::{courses_client::CoursesClient, CreateAnchorRequest, CreateAnchorResponse},
    shared::User,
};

use crate::{entities::CreateAnchor, errors::GatewayError};

pub async fn create_anchor(
    user: Option<User>,
    data: CreateAnchor,
    channel: tonic::transport::Channel,
) -> Result<CreateAnchorResponse, GatewayError> {
    let mut client = CoursesClient::new(channel);
    let request = tonic::Request::new(CreateAnchorRequest {
        active_user: user,
        title: data.title,
        track_time: data.track_time as f32,
        position_top: data.position_top as f32,
        position_left: data.position_left as f32,
        page_id: data.page_id,
        track_id: data.track_id,
    });
    let response = client.create_anchor(request).await?.into_inner();
    Ok(response)
}
