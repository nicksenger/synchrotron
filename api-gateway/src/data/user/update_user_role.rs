use schema::{
    shared::User,
    users::{users_client::UsersClient, UpdateUserRoleRequest, UpdateUserRoleResponse},
};

use crate::{entities::UserRole, errors::GatewayError};

pub async fn update_user_role(
    user: Option<User>,
    user_id: i32,
    new_role: UserRole,
    channel: tonic::transport::Channel,
) -> Result<UpdateUserRoleResponse, GatewayError> {
    let mut client = UsersClient::new(channel);
    let request = tonic::Request::new(UpdateUserRoleRequest {
        active_user: user,
        user_id: user_id,
        new_role: match new_role {
            UserRole::Standard => 0,
            UserRole::Moderator => 1,
            UserRole::Administrator => 2,
        },
    });
    let response = client.update_user_role(request).await?.into_inner();
    Ok(response)
}
