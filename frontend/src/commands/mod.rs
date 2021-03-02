use iced::Command;

mod application;
mod authentication;
mod operations;
mod routing;
mod ui;

use crate::{messages::Msg, state::Model};

pub fn get_command(message: &Msg, state: &Model) -> Command<Msg> {
    match message {
        Msg::Application(x) => application::get_command(x, state),
        Msg::Authentication(x) => authentication::get_command(x),
        Msg::Routing(x) => routing::get_command(x),
        Msg::Ui(x) => ui::get_command(x, state),
        _ => Command::none(),
    }
}
