use schema::{
    courses::{courses_client::CoursesClient, DeleteUserAnchorRequest, DeleteUserAnchorResponse},
    shared::User,
};

use crate::errors::GatewayError;

pub async fn delete_user_anchor(
    user: Option<User>,
    anchor_id: i32,
    channel: tonic::transport::Channel,
) -> Result<DeleteUserAnchorResponse, GatewayError> {
    let mut client = CoursesClient::new(channel);
    let request = tonic::Request::new(DeleteUserAnchorRequest {
        active_user: user,
        id: anchor_id,
    });
    let response = client.delete_user_anchor(request).await?.into_inner();
    Ok(response)
}
