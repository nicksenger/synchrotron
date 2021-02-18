use schema::courses::{courses_client::CoursesClient, GetPageAnchorsRequest};

use crate::{entities::Anchor, errors::GatewayError};

pub async fn page_anchors(
    channel: tonic::transport::Channel,
    page_id: i32,
) -> Result<Vec<Anchor>, GatewayError> {
    let mut client = CoursesClient::new(channel);
    let request = tonic::Request::new(GetPageAnchorsRequest {
        page_id,
    });
    let result = client.get_page_anchors(request).await?.into_inner();
    Ok(result.anchors.into_iter().map(|a| a.into()).collect())
}
