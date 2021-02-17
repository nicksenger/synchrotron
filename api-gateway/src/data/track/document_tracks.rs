use schema::courses::{courses_client::CoursesClient, GetDocumentTracksRequest};

use crate::{entities::Track, errors::GatewayError};

pub async fn document_tracks(
    channel: tonic::transport::Channel,
    document_id: i32,
    limit: i32,
    offset: i32,
) -> Result<Vec<Track>, GatewayError> {
    let mut client = CoursesClient::new(channel);
    let request = tonic::Request::new(GetDocumentTracksRequest {
        document_id,
        limit,
        offset,
    });
    let result = client.get_document_tracks(request).await?.into_inner();
    Ok(result.tracks.into_iter().map(|t| t.into()).collect())
}
