use std::convert::From;

use crate::graphql::schema::Context;

#[derive(Debug, Clone)]
/// A Microbiome user
pub struct User {
    // ID of the user
    pub id: i32,
    // Username of the user
    pub username: String,
    // Role of the user
    pub role: UserRole,
}

#[derive(juniper::GraphQLInputObject, Debug, Clone)]
// Creating a new user
pub struct NewUser {
    // Username for the new user
    pub username: String,
    // Password for the new user
    pub password: String,
}

#[derive(juniper::GraphQLEnum, Debug, Clone)]
pub enum UserRole {
    // A standard user in the system: has privileges for everyday functionality
    Standard,
    // A moderator: in addition to the standard privileges, may manage global data in the system
    Moderator,
    // An administrator: has full system capabilities
    Administrator,
}

#[derive(juniper::GraphQLInputObject, Debug, Clone)]
// Updating a user's role
pub struct UpdateUserRole {
    // ID of the user whose role should be updated
    pub user_id: i32,
    // New role for the user
    pub new_role: UserRole,
}

#[derive(juniper::GraphQLObject, Debug, Clone)]
/// Response to updating a user's role
pub struct UpdateUserRoleResponse {
    // Success flag
    pub success: bool,
}

#[derive(juniper::GraphQLInputObject, Debug, Clone)]
// Logging in as a user
pub struct Login {
    // Username for the new user
    pub username: String,
    // Password for the new user
    pub password: String,
}

#[derive(Debug, Clone)]
/// Response to logging in
pub struct LoginResponse {
    // Token
    pub token: String,
}

#[juniper::graphql_object(Context = Context)]
impl User {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn username(&self) -> &str {
        self.username.as_str()
    }
}

impl From<schema::shared::User> for User {
    fn from(x: schema::shared::User) -> User {
        User {
            id: x.id,
            username: x.username,
            role: match x.role {
                1 => UserRole::Moderator,
                2 => UserRole::Administrator,
                _ => UserRole::Standard,
            },
        }
    }
}

#[juniper::graphql_object(Context = Context)]
impl LoginResponse {
    pub fn token(&self) -> &str {
        self.token.as_str()
    }
}
