use schema::courses::{courses_client::CoursesClient, GetDocumentsRequest};

use crate::{entities::Document, errors::GatewayError};

pub async fn all_documents(
    channel: tonic::transport::Channel,
    limit: i32,
    offset: i32,
) -> Result<Vec<Document>, GatewayError> {
    let mut client = CoursesClient::new(channel);
    let request = tonic::Request::new(GetDocumentsRequest { limit, offset });
    let result = client.get_documents(request).await?.into_inner();
    Ok(result.documents.into_iter().map(|d| d.into()).collect())
}
