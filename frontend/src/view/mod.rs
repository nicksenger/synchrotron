use iced::{Element, Text};

use super::{
    messages::Msg,
    state::{Model, Route},
};

mod course;
mod courses;
mod login;
mod register;

pub fn view(state: &mut Model) -> Element<Msg> {
    match state.routing.route {
        Route::Login => login::view(state),
        Route::Register => register::view(state),
        Route::Courses => courses::view(state),
        Route::Course(document_id, _) => course::view(state, document_id),
        _ => Text::new("test").into(),
    }
}
