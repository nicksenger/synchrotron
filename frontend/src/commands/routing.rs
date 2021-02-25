use iced::Command;

use crate::messages::{routing, Msg};

pub fn get_command(msg: &routing::Msg) -> Command<Msg> {
    match msg {
        _ => Command::none(),
    }
}
