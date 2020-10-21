use std::collections::HashMap;

use async_trait::async_trait;
use dataloader::{cached::Loader, BatchFn};
use schema::users::{users_client::UsersClient, GetUsersByIdsRequest};
use tonic::transport::Channel;

use crate::{entities::User, errors::GatewayError};

async fn get_user_by_ids(
    map: &mut HashMap<i32, User>,
    user_ids: Vec<i32>,
    mut client: UsersClient<Channel>,
) -> Result<(), GatewayError> {
    let request = tonic::Request::new(GetUsersByIdsRequest { user_ids });
    let response = client.get_users_by_ids(request).await?.into_inner();

    for u in response.users {
        map.insert(u.id, u.into());
    }

    Ok(())
}

pub struct UserBatcher {
    channel: Channel,
}

impl UserBatcher {
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

#[async_trait]
impl BatchFn<i32, User> for UserBatcher {
    async fn load(&self, keys: &[i32]) -> HashMap<i32, User> {
        let client = UsersClient::new(self.channel.clone());

        let mut anchor_map = HashMap::new();
        get_user_by_ids(&mut anchor_map, keys.to_vec(), client).await;
        anchor_map
    }
}

pub type UserLoader = Loader<i32, User, UserBatcher>;

pub fn get_loader(channel: Channel) -> UserLoader {
    Loader::new(UserBatcher::new(channel))
}
