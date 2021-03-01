use crate::messages::{authentication, ui, Msg};

#[derive(Default)]
pub struct Model {
    pub loading: bool,
    pub username_input_value: String,
    pub password_input_value: String,
}

impl Model {
    pub fn update(&mut self, message: &Msg) {
        match message {
            Msg::Ui(ui::Msg::Login(ui::login::Msg::UsernameInputChanged(val))) => {
                self.username_input_value = val.to_owned();
            }
            Msg::Ui(ui::Msg::Login(ui::login::Msg::PasswordInputChanged(val))) => {
                self.password_input_value = val.to_owned();
            }
            Msg::Authentication(authentication::Msg::LoginRequest(_)) => {
                self.loading = true;
            }
            Msg::Authentication(authentication::Msg::LoginResponse(_)) => {
                self.loading = false;
            }
            _ => {}
        }
    }
}
