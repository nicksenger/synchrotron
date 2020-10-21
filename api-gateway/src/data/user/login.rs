use schema::users::{users_client::UsersClient, AuthenticateRequest};

use crate::{entities::Login, errors::GatewayError};

pub async fn login(
    data: Login,
    channel: tonic::transport::Channel,
) -> Result<String, GatewayError> {
    let mut client = UsersClient::new(channel);
    let request = tonic::Request::new(AuthenticateRequest {
        username: data.username,
        password: data.password,
    });
    let response = client.authenticate(request).await?.into_inner();
    Ok(response.token)
}
