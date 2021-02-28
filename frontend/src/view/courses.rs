use iced::{Button, Column, Element, Row, Text};

use crate::{
    messages::{routing, Msg},
    state::{Model, Route},
};

pub fn view(state: &mut Model) -> Element<Msg> {
    if state.ui.courses_screen.loading {
        return Text::new("loading...").into();
    }

    let mut el = Column::new();

    for (document_id, button_state) in state.ui.courses_screen.btn_states.iter_mut() {
        let d = state.entities.documents_by_id.get(document_id).unwrap();
        el = el.push(
            Row::new().push(
                Button::new(
                    button_state,
                    Text::new(d.title.as_str()),
                )
                .on_press(Msg::Routing(routing::Msg::Navigate(Route::Course(
                    d.id, None,
                )))),
            ),
        );
    }

    el.into()
}
