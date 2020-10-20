use schema::users::{users_client::UsersClient, VerifyRequest};

pub async fn verify(token: String, channel: tonic::transport::Channel) -> Option<i32> {
    let mut client = UsersClient::new(channel);
    let request = tonic::Request::new(VerifyRequest { token });
    client
        .verify(request)
        .await
        .map(|res| res.into_inner().user_id)
        .ok()
}
