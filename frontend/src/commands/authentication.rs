use iced::Command;

use super::operations;
use crate::messages::{authentication, Msg};

pub fn get_command(msg: &authentication::Msg) -> Command<Msg> {
    match msg {
        authentication::Msg::LoginRequest(payload) => {
            Command::perform(operations::login(payload.into()), |x| {
                Msg::Authentication(authentication::Msg::LoginResponse(x))
            })
        }
        // authentication::Msg::LoginResponse(response) => {

        // },
        // authentication::Msg::RegisterRequest(payload) => {

        // },
        // authentication::Msg::RegisterResponse(response) => {

        // },
        _ => Command::none(),
    }
}
