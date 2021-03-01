use iced_web::{dodrio, dodrio::bumpalo, Bus};

use crate::{
    messages::{routing, Msg},
    state::{Model, Route},
};

pub fn render<'b, 's>(
    bump: &'b bumpalo::Bump,
    state: &'s Model,
    bus: &Bus<Msg>,
) -> dodrio::Node<'b> {
    use dodrio::builder::*;
    if state.ui.courses_screen.loading {
        return p(bump)
            .child(text(
                dodrio::bumpalo::collections::String::from_str_in("Loading...", bump)
                    .into_bump_str(),
            ))
            .finish();
    }

    let ids_titles = state
        .entities
        .documents_by_id
        .iter()
        .map(|(&document_id, document)| (document_id, document.title.clone()));

    div::<'b>(bump)
        .children(bumpalo::collections::Vec::from_iter_in(
            ids_titles.map(|(document_id, title)| {
                let button_bus = bus.clone();

                p(bump).child(
                    button::<'b>(bump)
                        .child(text(
                            dodrio::bumpalo::collections::String::from_str_in(title.as_str(), bump)
                                .into_bump_str(),
                        ))
                        .on("click", move |_root, _vdom, event| {
                            button_bus.publish(Msg::Routing(routing::Msg::Navigate(
                                Route::Course(document_id, None),
                            )));
                        })
                        .finish(),
                ).finish()
            }),
            bump,
        ))
        .finish()
}
