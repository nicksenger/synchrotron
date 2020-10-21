use schema::users::{users_client::UsersClient, VerifyRequest};

use crate::errors::GatewayError;

pub async fn verify(token: String, channel: tonic::transport::Channel) -> Result<i32, GatewayError> {
    let mut client = UsersClient::new(channel);
    let request = tonic::Request::new(VerifyRequest { token });
    let id = client
        .verify(request)
        .await?
        .map(|res| res.user_id)
        .into_inner();
    
    Ok(id)
}
