use schema::{
    courses::{courses_client::CoursesClient, DeleteBookmarkRequest},
    shared::User,
};

use crate::{entities::DeleteBookmarkResponse, errors::GatewayError};

pub async fn delete_bookmark(
    channel: tonic::transport::Channel,
    bookmark_id: i32,
    active_user: Option<User>,
) -> Result<DeleteBookmarkResponse, GatewayError> {
    let mut client = CoursesClient::new(channel);
    let request = tonic::Request::new(DeleteBookmarkRequest {
        bookmark_id,
        active_user,
    });
    let _ = client.delete_bookmark(request).await?.into_inner();
    Ok(DeleteBookmarkResponse { success: true })
}
