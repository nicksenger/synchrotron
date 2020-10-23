use crate::{
    entities::{Login, NewUser, User},
    errors::GatewayError,
};

mod all_users;
mod create_user;
mod get_user_by_id;
use get_user_by_id::{get_loader, UserLoader};
mod authenticate;
mod login;

#[derive(Clone)]
pub struct UserData {
    channel: tonic::transport::Channel,
    user_by_id: UserLoader,
}

impl UserData {
    pub fn new(channel: tonic::transport::Channel) -> Self {
        Self {
            user_by_id: get_loader(channel.clone()),
            channel,
        }
    }

    pub async fn user_by_id(&self, id: i32) -> User {
        self.user_by_id.load(id).await
    }

    pub async fn create_user(&self, data: NewUser) -> Result<User, GatewayError> {
        create_user::create_user(data, self.channel.clone()).await
    }

    pub async fn login(&self, data: Login) -> Result<String, GatewayError> {
        login::login(data, self.channel.clone()).await
    }

    pub async fn all_users(&self) -> Result<Vec<User>, GatewayError> {
        all_users::all_users(self.channel.clone()).await
    }

    pub async fn authenticate(&self, token: String) -> Result<i32, GatewayError> {
        authenticate::authenticate(token, self.channel.clone()).await
    }
}
