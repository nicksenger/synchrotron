use schema::users::{users_client::UsersClient, AuthenticateRequest};

use crate::entities::Login;

pub async fn login(data: Login, channel: tonic::transport::Channel) -> String {
    let mut client = UsersClient::new(channel);
    let request = tonic::Request::new(AuthenticateRequest {
        username: data.username,
        password: data.password,
    });
    let response = client.authenticate(request).await.unwrap().into_inner();
    response.token
}
