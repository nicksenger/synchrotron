use schema::{
    users::{users_client::UsersClient, GetTokenRequest},
};

use crate::entities::User;
use crate::errors::GatewayError;

pub async fn login(
    username: String,
    password: String,
    channel: tonic::transport::Channel,
) -> Result<(String, User), GatewayError> {
    let mut client = UsersClient::new(channel);
    let request = tonic::Request::new(GetTokenRequest {
        username: username,
        password: password,
    });
    let response = client.get_token(request).await?.into_inner();
    Ok((response.token, response.user.unwrap().into()))
}
