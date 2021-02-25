use std::collections::HashMap;

use async_trait::async_trait;
use dataloader::{cached::Loader, BatchFn};
use schema::courses::{courses_client::CoursesClient, GetAnchorsByPageIDsRequest};
use tonic::transport::Channel;

use crate::{entities::Anchor, errors::GatewayError};

async fn get_anchors_by_page_id(
    map: &mut HashMap<i32, Vec<Anchor>>,
    ids: Vec<i32>,
    mut client: CoursesClient<Channel>,
) -> Result<(), GatewayError> {
    let request = tonic::Request::new(GetAnchorsByPageIDsRequest { ids });
    let response = client.get_anchors_by_page_ids(request).await?.into_inner();

    for (page_id, page_anchors) in response.anchors {
        map.insert(
            page_id,
            page_anchors.anchors.into_iter().map(|a| a.into()).collect(),
        );
    }

    Ok(())
}

pub struct PageAnchorBatcher {
    channel: Channel,
}

impl PageAnchorBatcher {
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

#[async_trait]
impl BatchFn<i32, Vec<Anchor>> for PageAnchorBatcher {
    async fn load(&mut self, keys: &[i32]) -> HashMap<i32, Vec<Anchor>> {
        let client = CoursesClient::new(self.channel.clone());

        let mut page_anchor_map = HashMap::new();
        keys.iter().for_each(|&k| {
            page_anchor_map.insert(k, vec![]);
        });
        let _ = get_anchors_by_page_id(&mut page_anchor_map, keys.to_vec(), client).await;
        page_anchor_map
    }
}

pub type PageAnchorLoader = Loader<i32, Vec<Anchor>, PageAnchorBatcher>;

pub fn get_page_loader(channel: Channel) -> PageAnchorLoader {
    Loader::new(PageAnchorBatcher::new(channel))
}
