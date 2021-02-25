use iced::Command;

use crate::messages::{application, Msg};

pub fn get_command(msg: &application::Msg) -> Command<Msg> {
    match msg {
        _ => Command::none(),
    }
}
