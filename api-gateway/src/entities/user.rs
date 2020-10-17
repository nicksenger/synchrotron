use crate::{graphql::schema::Context};

#[derive(Debug, Clone)]
/// A Glot user
pub struct User {
    /// Name of the user
    pub username: String,
}

#[juniper::graphql_object(Context = Context)]
impl User {
    pub fn username(&self) -> &str {
        self.username.as_str()
    }
}
