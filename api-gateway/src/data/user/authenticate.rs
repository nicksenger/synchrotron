use schema::users::{users_client::UsersClient, AuthenticateRequest};

use crate::errors::GatewayError;

pub async fn authenticate(
    token: String,
    channel: tonic::transport::Channel,
) -> Result<i32, GatewayError> {
    let mut client = UsersClient::new(channel);
    let request = tonic::Request::new(AuthenticateRequest { token });
    let id = client
        .authenticate(request)
        .await?
        .map(|res| res.user.unwrap().id)
        .into_inner();

    Ok(id)
}
