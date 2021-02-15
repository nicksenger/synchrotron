use schema::users::{users_client::UsersClient, AuthenticateRequest, User};

use crate::errors::GatewayError;

pub async fn authenticate(
    token: String,
    channel: tonic::transport::Channel,
) -> Result<User, GatewayError> {
    let mut client = UsersClient::new(channel);
    let request = tonic::Request::new(AuthenticateRequest { token });
    let result = client
        .authenticate(request)
        .await?
        .into_inner().user.unwrap();

    Ok(result)
}
