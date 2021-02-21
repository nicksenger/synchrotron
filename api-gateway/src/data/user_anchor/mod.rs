use crate::{
    entities::{CreateUserAnchor, DeleteUserAnchorResponse, UserAnchor},
    errors::GatewayError,
};
use schema::shared::User;

mod create_user_anchor;
mod delete_user_anchor;
mod user_anchors_by_id;
mod user_anchors_by_page_id;

use user_anchors_by_id::{get_loader, UserAnchorLoader};
use user_anchors_by_page_id::{get_page_loader, PageUserAnchorLoader};

#[derive(Clone)]
pub struct UserAnchorData {
    channel: tonic::transport::Channel,
    user_anchors_by_id: UserAnchorLoader,
    user_anchors_by_page_id: PageUserAnchorLoader,
}

impl UserAnchorData {
    pub fn new(channel: tonic::transport::Channel) -> Self {
        Self {
            user_anchors_by_id: get_loader(channel.clone()),
            user_anchors_by_page_id: get_page_loader(channel.clone()),
            channel,
        }
    }

    pub async fn user_anchors_by_id(&self, id: i32) -> UserAnchor {
        self.user_anchors_by_id.load(id).await
    }

    pub async fn page_user_anchors(&self, page_id: i32) -> Vec<UserAnchor> {
        self.user_anchors_by_page_id.load(page_id).await
    }

    pub async fn create_user_anchor(
        &self,
        user: Option<User>,
        data: CreateUserAnchor,
    ) -> Result<UserAnchor, GatewayError> {
        let response =
            create_user_anchor::create_user_anchor(user, data, self.channel.clone()).await?;
        Ok(response.user_anchor.unwrap().into())
    }

    pub async fn delete_user_anchor(
        &self,
        user: Option<User>,
        user_anchor_id: i32,
    ) -> Result<DeleteUserAnchorResponse, GatewayError> {
        let response =
            delete_user_anchor::delete_user_anchor(user, user_anchor_id, self.channel.clone())
                .await?;
        Ok(DeleteUserAnchorResponse {
            success: response.success,
        })
    }
}
