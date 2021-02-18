use schema::{
    courses::{courses_client::CoursesClient, DeleteAnchorRequest, DeleteAnchorResponse},
    shared::User,
};

use crate::errors::GatewayError;

pub async fn delete_anchor(
    user: Option<User>,
    anchor_id: i32,
    channel: tonic::transport::Channel,
) -> Result<DeleteAnchorResponse, GatewayError> {
    let mut client = CoursesClient::new(channel);
    let request = tonic::Request::new(DeleteAnchorRequest {
        active_user: user,
        id: anchor_id,
    });
    let response = client.delete_anchor(request).await?.into_inner();
    Ok(response)
}
