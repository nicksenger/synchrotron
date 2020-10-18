use crate::entities::{User, NewUser};

mod create_user;
mod get_user_by_id;
use get_user_by_id::{get_loader, UserLoader};

#[derive(Clone)]
pub struct UserData {
    channel: tonic::transport::Channel,
    user_by_id: UserLoader,
}

impl UserData {
    pub fn new(channel: tonic::transport::Channel) -> Self {
        Self {
            user_by_id: get_loader(channel.clone()),
            channel
        }
    }

    pub async fn user_by_id(&self, id: i32) -> User {
        self.user_by_id.load(id).await
    }

    pub async fn create_user(&self, data: NewUser) -> User {
        create_user::create_user(data, self.channel.clone()).await
    }
}
