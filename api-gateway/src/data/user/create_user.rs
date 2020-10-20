use schema::users::{users_client::UsersClient, CreateUserRequest};

use crate::entities::{NewUser, User};

pub async fn create_user(data: NewUser, channel: tonic::transport::Channel) -> User {
    let mut client = UsersClient::new(channel);
    let request = tonic::Request::new(CreateUserRequest {
        username: data.username,
        password: data.password,
    });
    let response = client.create_user(request).await.unwrap().into_inner();
    response.user.unwrap().into()
}
