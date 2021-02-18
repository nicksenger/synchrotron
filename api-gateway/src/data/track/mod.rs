use crate::{
  entities::{Track, DocumentTracks},
  errors::GatewayError,
};

mod tracks_by_id;
mod document_tracks;

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
      data: DocumentTracks,
  ) -> Result<Vec<Track>, GatewayError> {
      document_tracks::document_tracks(
          self.channel.clone(),
          data.document_id,
          data.limit,
          data.offset,
      )
      .await
  }
}
