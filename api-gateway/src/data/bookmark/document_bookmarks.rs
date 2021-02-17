use schema::courses::{courses_client::CoursesClient, GetDocumentBookmarksRequest};

use crate::{entities::Bookmark, errors::GatewayError};

pub async fn document_bookmarks(
    channel: tonic::transport::Channel,
    document_id: i32,
    limit: i32,
    offset: i32,
) -> Result<Vec<Bookmark>, GatewayError> {
    let mut client = CoursesClient::new(channel);
    let request = tonic::Request::new(GetDocumentBookmarksRequest {
        document_id,
        limit,
        offset,
    });
    let result = client.get_document_bookmarks(request).await?.into_inner();
    Ok(result.bookmarks.into_iter().map(|b| b.into()).collect())
}
