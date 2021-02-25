use std::collections::HashMap;

use async_trait::async_trait;
use dataloader::{cached::Loader, BatchFn};
use schema::courses::{courses_client::CoursesClient, GetAnchorsByIDsRequest};
use tonic::transport::Channel;

use crate::{entities::Anchor, errors::GatewayError};

async fn get_anchor_by_id(
    map: &mut HashMap<i32, Anchor>,
    ids: Vec<i32>,
    mut client: CoursesClient<Channel>,
) -> Result<(), GatewayError> {
    let request = tonic::Request::new(GetAnchorsByIDsRequest { ids });
    let response = client.get_anchors_by_ids(request).await?.into_inner();

    for a in response.anchors {
        map.insert(a.id, a.into());
    }

    Ok(())
}

pub struct AnchorBatcher {
    channel: Channel,
}

impl AnchorBatcher {
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

#[async_trait]
impl BatchFn<i32, Anchor> for AnchorBatcher {
    async fn load(&mut self, keys: &[i32]) -> HashMap<i32, Anchor> {
        let client = CoursesClient::new(self.channel.clone());

        let mut anchor_map = HashMap::new();
        let _ = get_anchor_by_id(&mut anchor_map, keys.to_vec(), client).await;
        anchor_map
    }
}

pub type AnchorLoader = Loader<i32, Anchor, AnchorBatcher>;

pub fn get_loader(channel: Channel) -> AnchorLoader {
    Loader::new(AnchorBatcher::new(channel))
}
