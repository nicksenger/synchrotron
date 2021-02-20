use crate::{
    entities::{Anchor, CreateAnchor, DeleteAnchorResponse},
    errors::GatewayError,
};
use schema::shared::User;

mod anchors_by_id;
mod create_anchor;
mod delete_anchor;
mod page_anchors;

use anchors_by_id::{get_loader, AnchorLoader};

#[derive(Clone)]
pub struct AnchorData {
    channel: tonic::transport::Channel,
    anchors_by_id: AnchorLoader,
}

impl AnchorData {
    pub fn new(channel: tonic::transport::Channel) -> Self {
        Self {
            anchors_by_id: get_loader(channel.clone()),
            channel,
        }
    }

    pub async fn anchors_by_id(&self, id: i32) -> Anchor {
        self.anchors_by_id.load(id).await
    }

    pub async fn page_anchors(&self, page_id: i32) -> Result<Vec<Anchor>, GatewayError> {
        page_anchors::page_anchors(self.channel.clone(), page_id).await
    }

    pub async fn create_anchor(
        &self,
        user: Option<User>,
        data: CreateAnchor,
    ) -> Result<Anchor, GatewayError> {
        let response = create_anchor::create_anchor(user, data, self.channel.clone()).await?;
        Ok(response.anchor.unwrap().into())
    }

    pub async fn delete_anchor(
        &self,
        user: Option<User>,
        anchor_id: i32,
    ) -> Result<DeleteAnchorResponse, GatewayError> {
        let response = delete_anchor::delete_anchor(user, anchor_id, self.channel.clone()).await?;
        Ok(DeleteAnchorResponse {
            success: response.success,
        })
    }
}
