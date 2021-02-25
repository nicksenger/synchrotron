use std::collections::HashMap;

use async_trait::async_trait;
use dataloader::{cached::Loader, BatchFn};
use schema::courses::{courses_client::CoursesClient, GetTracksByIDsRequest};
use tonic::transport::Channel;

use crate::{entities::Track, errors::GatewayError};

async fn get_track_by_id(
    map: &mut HashMap<i32, Track>,
    ids: Vec<i32>,
    mut client: CoursesClient<Channel>,
) -> Result<(), GatewayError> {
    let request = tonic::Request::new(GetTracksByIDsRequest { ids });
    let response = client.get_tracks_by_ids(request).await?.into_inner();

    for p in response.tracks {
        map.insert(p.id, p.into());
    }

    Ok(())
}

pub struct TrackBatcher {
    channel: Channel,
}

impl TrackBatcher {
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

#[async_trait]
impl BatchFn<i32, Track> for TrackBatcher {
    async fn load(&mut self, keys: &[i32]) -> HashMap<i32, Track> {
        let client = CoursesClient::new(self.channel.clone());

        let mut anchor_map = HashMap::new();
        let _ = get_track_by_id(&mut anchor_map, keys.to_vec(), client).await;
        anchor_map
    }
}

pub type TrackLoader = Loader<i32, Track, TrackBatcher>;

pub fn get_loader(channel: Channel) -> TrackLoader {
    Loader::new(TrackBatcher::new(channel))
}
