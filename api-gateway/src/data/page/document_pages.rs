use schema::courses::{courses_client::CoursesClient, GetDocumentPagesRequest};

use crate::{entities::Page, errors::GatewayError};

pub async fn document_pages(
    channel: tonic::transport::Channel,
    document_id: i32,
    limit: i32,
    offset: i32,
) -> Result<Vec<Page>, GatewayError> {
    let mut client = CoursesClient::new(channel);
    let request = tonic::Request::new(GetDocumentPagesRequest {
        document_id,
        limit,
        offset,
    });
    let result = client.get_document_pages(request).await?.into_inner();
    Ok(result.pages.into_iter().map(|b| b.into()).collect())
}
