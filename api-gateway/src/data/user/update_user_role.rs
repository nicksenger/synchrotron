use schema::users::{users_client::UsersClient, UpdateUserRoleRequest, UpdateUserRoleResponse, User};

use crate::{entities::{UpdateUserRole, UserRole}, errors::GatewayError};

pub async fn update_user_role(
    user: Option<User>,
    data: UpdateUserRole,
    channel: tonic::transport::Channel,
) -> Result<UpdateUserRoleResponse, GatewayError> {
    let mut client = UsersClient::new(channel);
    let request = tonic::Request::new(UpdateUserRoleRequest {
        active_user: user,
        user_id: data.user_id,
        new_role: match data.new_role {
          UserRole::Standard => 0,
          UserRole::Moderator => 1,
          UserRole::Administrator => 2,
        },
    });
    let response = client.update_user_role(request).await?.into_inner();
    Ok(response)
}
