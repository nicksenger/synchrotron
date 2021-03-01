use iced_web::{dodrio, Bus, Css, Element, Widget};

use super::{
    messages::Msg,
    state::{Model, Route},
};

mod course;
mod courses;
mod login;
mod register;

pub struct View<'a> {
    state: &'a Model,
}

impl<'a> View<'a> {
    pub fn new(state: &'a Model) -> Self {
        Self { state }
    }
}

impl<'a> Widget<Msg> for View<'a> {
    fn node<'b>(
        &self,
        bump: &'b dodrio::bumpalo::Bump,
        bus: &Bus<Msg>,
        style_sheet: &mut Css<'b>,
    ) -> dodrio::Node<'b> {
        use dodrio::builder::*;

        match self.state.routing.route {
            Route::NotFound => p(bump).child(text(
                dodrio::bumpalo::collections::String::from_str_in(
                    "Oops, the requested page does not exist!",
                    bump,
                )
                .into_bump_str(),
            )).finish(),
            Route::Login => login::render(bump, self.state, bus),
            Route::Register => register::render(bump, self.state, bus),
            Route::Courses => courses::render(bump, self.state, bus),
            Route::Course(document_id, _) => course::render(bump, self.state, bus, document_id),
        }
    }
}

impl<'a> From<View<'a>> for Element<'a, Msg> {
    fn from(view: View<'a>) -> Element<'a, Msg> {
        Element::new(view)
    }
}
