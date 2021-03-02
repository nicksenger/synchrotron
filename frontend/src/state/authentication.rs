use crate::messages::{authentication, Msg};

#[derive(Default)]
pub struct Model {
    pub active_user: Option<i32>,
    pub token: Option<String>,
}

impl Model {
    pub fn update(&mut self, message: &Msg) {
        match message {
            Msg::Authentication(authentication::Msg::Logout) => {
                self.active_user = None;
                self.token = None;
            }
            Msg::Authentication(authentication::Msg::LoginResponse(Ok(payload))) => {
                self.active_user = Some(payload.user.id);
                self.token = Some(payload.token.clone());
            }
            _ => {}
        }
    }
}
