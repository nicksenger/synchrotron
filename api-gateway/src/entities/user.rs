use std::convert::From;

use crate::graphql::schema::Context;

#[derive(Debug, Clone)]
/// A Microbiome user
pub struct User {
    // ID of the user
    pub id: i32,
    /// Usrname of the user
    pub username: String,
}

#[derive(juniper::GraphQLInputObject, Debug, Clone)]
// Creating a new user
pub struct NewUser {
    // Username for the new user
    pub username: String,
    // Password for the new user
    pub password: String,
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

impl From<schema::users::User> for User {
    fn from(x: schema::users::User) -> User {
        User {
            id: x.id,
            username: x.username,
        }
    }
}

#[juniper::graphql_object(Context = Context)]
impl LoginResponse {
    pub fn token(&self) -> &str {
        self.token.as_str()
    }
}
