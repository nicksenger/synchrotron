use schema::users::{users_client::UsersClient, GetAllUsersRequest};

use crate::entities::User;

pub async fn all_users(channel: tonic::transport::Channel) -> Vec<User> {
    let mut client = UsersClient::new(channel);
    let request = tonic::Request::new(GetAllUsersRequest {});
    let mut stream = client.get_all_users(request).await.unwrap().into_inner();
    let mut users = vec![];
    while let Some(user) = stream.message().await.unwrap() {
        users.push(user.into());
    }
    users
}
