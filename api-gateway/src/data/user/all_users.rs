use schema::users::{users_client::UsersClient, GetAllUsersRequest};

use crate::{entities::User, errors::GatewayError};

pub async fn all_users(channel: tonic::transport::Channel) -> Result<Vec<User>, GatewayError> {
    let mut client = UsersClient::new(channel);
    let request = tonic::Request::new(GetAllUsersRequest {});
    let mut stream = client.get_all_users(request).await?.into_inner();
    let mut users = vec![];
    while let Some(user) = stream.message().await? {
        users.push(user.into());
    }
    Ok(users)
}
