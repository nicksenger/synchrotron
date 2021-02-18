use schema::courses::{courses_client::CoursesClient, GetPageUserAnchorsRequest};

use crate::{entities::UserAnchor, errors::GatewayError};

pub async fn page_user_anchors(
    channel: tonic::transport::Channel,
    page_id: i32,
) -> Result<Vec<UserAnchor>, GatewayError> {
    let mut client = CoursesClient::new(channel);
    let request = tonic::Request::new(GetPageUserAnchorsRequest { page_id });
    let result = client.get_page_user_anchors(request).await?.into_inner();
    Ok(result.user_anchors.into_iter().map(|a| a.into()).collect())
}
