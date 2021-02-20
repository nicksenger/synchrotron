use crate::{
    entities::{NewUser, User, UserRole},
    errors::GatewayError,
};

mod all_users;
mod create_user;
mod get_user_by_id;
use get_user_by_id::{get_loader, UserLoader};
use schema::users::UpdateUserRoleResponse;
mod authenticate;
mod login;
mod update_user_role;

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

    pub async fn login(&self, username: String, password: String) -> Result<String, GatewayError> {
        login::login(username, password, self.channel.clone()).await
    }

    pub async fn all_users(&self) -> Result<Vec<User>, GatewayError> {
        all_users::all_users(self.channel.clone()).await
    }

    pub async fn authenticate(&self, token: String) -> Result<schema::shared::User, GatewayError> {
        authenticate::authenticate(token, self.channel.clone()).await
    }

    pub async fn update_user_role(
        &self,
        user_id: i32,
        new_role: UserRole,
        user: Option<schema::shared::User>,
    ) -> Result<UpdateUserRoleResponse, GatewayError> {
        update_user_role::update_user_role(user, user_id, new_role, self.channel.clone()).await
    }
}
