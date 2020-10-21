use schema::users::{users_client::UsersClient, CreateUserRequest};

use crate::{
    entities::{NewUser, User},
    errors::GatewayError,
};

pub async fn create_user(
    data: NewUser,
    channel: tonic::transport::Channel,
) -> Result<User, GatewayError> {
    let mut client = UsersClient::new(channel);
    let request = tonic::Request::new(CreateUserRequest {
        username: data.username,
        password: data.password,
    });
    let response = client.create_user(request).await?.into_inner();
    Ok(response.user.unwrap().into())
}
