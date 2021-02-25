use iced::Command;

mod operations;
mod application;
mod authentication;
mod routing;

use crate::messages::Msg;

pub fn get_command(message: &Msg) -> Command<Msg> {
    match message {
        Msg::Application(x) => application::get_command(x),
        Msg::Authentication(x) => authentication::get_command(x),
        Msg::Routing(x) => routing::get_command(x),
        _ => Command::none(),
    }
}
