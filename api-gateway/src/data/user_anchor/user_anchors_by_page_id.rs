use std::collections::HashMap;

use async_trait::async_trait;
use dataloader::{cached::Loader, BatchFn};
use schema::courses::{courses_client::CoursesClient, GetUserAnchorsByPageIDsRequest};
use tonic::transport::Channel;

use crate::{entities::UserAnchor, errors::GatewayError};

async fn get_user_anchors_by_page_id(
    map: &mut HashMap<i32, Vec<UserAnchor>>,
    ids: Vec<i32>,
    mut client: CoursesClient<Channel>,
) -> Result<(), GatewayError> {
    let request = tonic::Request::new(GetUserAnchorsByPageIDsRequest { ids });
    let response = client
        .get_user_anchors_by_page_ids(request)
        .await?
        .into_inner();

    for (page_id, page_user_anchors) in response.user_anchors {
        map.insert(
            page_id,
            page_user_anchors
                .user_anchors
                .into_iter()
                .map(|a| a.into())
                .collect(),
        );
    }

    Ok(())
}

pub struct PageUserAnchorBatcher {
    channel: Channel,
}

impl PageUserAnchorBatcher {
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

#[async_trait]
impl BatchFn<i32, Vec<UserAnchor>> for PageUserAnchorBatcher {
    async fn load(&self, keys: &[i32]) -> HashMap<i32, Vec<UserAnchor>> {
        let client = CoursesClient::new(self.channel.clone());

        let mut page_anchor_map = HashMap::new();
        keys.iter().for_each(|&k| {
            page_anchor_map.insert(k, vec![]);
        });
        let _ = get_user_anchors_by_page_id(&mut page_anchor_map, keys.to_vec(), client).await;
        page_anchor_map
    }
}

pub type PageUserAnchorLoader = Loader<i32, Vec<UserAnchor>, PageUserAnchorBatcher>;

pub fn get_page_loader(channel: Channel) -> PageUserAnchorLoader {
    Loader::new(PageUserAnchorBatcher::new(channel))
}
