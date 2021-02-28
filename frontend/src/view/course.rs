use iced::{Column, Element, Image, Row, Text};

use crate::{messages::Msg, state::Model};

pub fn view(state: &mut Model, document_id: i32) -> Element<Msg> {
    if state.ui.course_screen.loading {
        return Text::new("loading...").into();
    }

    let mut el = Column::new();

    for page_id in state.entities.document_pages.get(&document_id).unwrap() {
        el = el.push(Row::new().push(Image::new(format!(
            "https://synchrotron.nsenger.com/{}",
            state.entities.pages_by_id.get(page_id).unwrap().image_path
        ))))
    }

    el.into()
}
