use schema::{
    courses::{courses_client::CoursesClient, CreateBookmarkRequest},
    shared::User,
};

use crate::{entities::Bookmark, errors::GatewayError};

pub async fn create_bookmark(
    channel: tonic::transport::Channel,
    title: String,
    page_id: i32,
    document_id: i32,
    active_user: Option<User>,
) -> Result<Bookmark, GatewayError> {
    let mut client = CoursesClient::new(channel);
    let request = tonic::Request::new(CreateBookmarkRequest {
        title,
        page_id,
        document_id,
        active_user,
    });
    let response = client.create_bookmark(request).await?.into_inner();
    Ok(response.bookmark.unwrap().into())
}
