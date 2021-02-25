use std::collections::HashMap;

use async_trait::async_trait;
use dataloader::{cached::Loader, BatchFn};
use schema::courses::{courses_client::CoursesClient, GetUserAnchorsByIDsRequest};
use tonic::transport::Channel;

use crate::{entities::UserAnchor, errors::GatewayError};

async fn get_anchor_by_id(
    map: &mut HashMap<i32, UserAnchor>,
    ids: Vec<i32>,
    mut client: CoursesClient<Channel>,
) -> Result<(), GatewayError> {
    let request = tonic::Request::new(GetUserAnchorsByIDsRequest { ids });
    let response = client.get_user_anchors_by_ids(request).await?.into_inner();

    for a in response.user_anchors {
        map.insert(a.id, a.into());
    }

    Ok(())
}

pub struct UserAnchorBatcher {
    channel: Channel,
}

impl UserAnchorBatcher {
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

#[async_trait]
impl BatchFn<i32, UserAnchor> for UserAnchorBatcher {
    async fn load(&mut self, keys: &[i32]) -> HashMap<i32, UserAnchor> {
        let client = CoursesClient::new(self.channel.clone());

        let mut anchor_map = HashMap::new();
        let _ = get_anchor_by_id(&mut anchor_map, keys.to_vec(), client).await;
        anchor_map
    }
}

pub type UserAnchorLoader = Loader<i32, UserAnchor, UserAnchorBatcher>;

pub fn get_loader(channel: Channel) -> UserAnchorLoader {
    Loader::new(UserAnchorBatcher::new(channel))
}
