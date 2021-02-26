use iced::{button, text_input};

use crate::messages::{Msg, ui};

#[derive(Default)]
pub struct Model {
    pub username_input_value: String,
    pub username_input_state: text_input::State,
    pub password_input_value: String,
    pub password_input_state: text_input::State,
    pub submit_button_state: button::State,
}

impl Model {
    pub fn update(&mut self, message: &Msg) {
        match message {
            Msg::Ui(ui::Msg::Login(ui::login::Msg::UsernameInputChanged(val))) => {
                self.username_input_value = val.to_owned();
            },
            Msg::Ui(ui::Msg::Login(ui::login::Msg::PasswordInputChanged(val))) => {
                self.password_input_value = val.to_owned();
            }
            _ => {}
        }
    }
}
