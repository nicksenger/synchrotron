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
            Msg::Ui(ui::Msg::Register(ui::register::Msg::UsernameInputChanged(val))) => {
                self.username_input_value = val.to_owned();
            }
            Msg::Ui(ui::Msg::Register(ui::register::Msg::PasswordInputChanged(val))) => {
                self.password_input_value = val.to_owned();
            }
            Msg::Authentication(authentication::Msg::RegisterRequest(_)) => {
                self.loading = true;
            }
            Msg::Authentication(authentication::Msg::RegisterResponse(_)) => {
                self.loading = false;
            }
            _ => {}
        }
    }
}
