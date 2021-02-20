use schema::shared::User;

use crate::{entities::Track, errors::GatewayError};

mod document_tracks;
mod tracks_by_id;
mod update_track_title;

use tracks_by_id::{get_loader, TrackLoader};

#[derive(Clone)]
pub struct TrackData {
    channel: tonic::transport::Channel,
    tracks_by_id: TrackLoader,
}

impl TrackData {
    pub fn new(channel: tonic::transport::Channel) -> Self {
        Self {
            tracks_by_id: get_loader(channel.clone()),
            channel,
        }
    }

    pub async fn tracks_by_id(&self, id: i32) -> Track {
        self.tracks_by_id.load(id).await
    }

    pub async fn document_tracks(
        &self,
        document_id: i32,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<Track>, GatewayError> {
        document_tracks::document_tracks(self.channel.clone(), document_id, limit, offset).await
    }

    pub async fn update_track_title(
        &self,
        track_id: i32,
        title: String,
        active_user: Option<User>,
    ) -> Result<Track, GatewayError> {
        update_track_title::update_track_title(active_user, track_id, title, self.channel.clone())
            .await
    }
}
