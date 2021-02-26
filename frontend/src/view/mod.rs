use iced::{Element, Text};

use super::{messages::Msg, state::{Model, Route}};

mod login;

pub fn view(state: &mut Model) -> Element<Msg> {
    match state.routing.route {
        Route::Login => login::view(state),
        _ => Text::new("test").into()
    }
}