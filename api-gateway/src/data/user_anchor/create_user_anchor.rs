use schema::{
    courses::{courses_client::CoursesClient, CreateUserAnchorRequest, CreateUserAnchorResponse},
    shared::User,
};

use crate::{entities::CreateUserAnchor, errors::GatewayError};

pub async fn create_user_anchor(
    user: Option<User>,
    data: CreateUserAnchor,
    channel: tonic::transport::Channel,
) -> Result<CreateUserAnchorResponse, GatewayError> {
    let mut client = CoursesClient::new(channel);
    let request = tonic::Request::new(CreateUserAnchorRequest {
        active_user: user,
        title: data.title,
        track_time: data.track_time as f32,
        position_top: data.position_top as f32,
        position_left: data.position_left as f32,
        page_id: data.page_id,
        track_id: data.track_id,
    });
    let response = client.create_user_anchor(request).await?.into_inner();
    Ok(response)
}
