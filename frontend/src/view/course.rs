use std::collections::HashSet;

use iced_web::{dodrio, dodrio::bumpalo, Bus};
use wasm_bindgen::JsCast;

use crate::{
    messages::{ui, Msg},
    state::Model,
};

pub fn render<'b>(
    bump: &'b bumpalo::Bump,
    state: &Model,
    bus: &Bus<Msg>,
    document_id: i32,
) -> dodrio::Node<'b> {
    use dodrio::builder::*;

    if state.ui.course_screen.loading {
        return p(bump)
            .child(text(
                dodrio::bumpalo::collections::String::from_str_in("Loading...", bump)
                    .into_bump_str(),
            ))
            .finish();
    }

    let scroll_bus = bus.clone();

    div::<'b>(bump)
        .attr("class", "synchrotron__inner")
        .children(bumpalo::collections::Vec::from_iter_in(
            vec![div(bump)
                .attr("id", "page-container")
                .attr("class", "synchrotron__page-container")
                .on("scroll", move |_root, _vdom, event| {
                    let el = match event
                        .target()
                        .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
                    {
                        None => return,
                        Some(el) => el,
                    };

                    scroll_bus.publish(Msg::Ui(ui::Msg::Course(
                        ui::course::Msg::UpdateRelativeScroll(
                            el.scroll_top() as f32 / el.client_width() as f32,
                        ),
                    )));
                })
                .children(bumpalo::collections::Vec::from_iter_in(
                    state
                        .entities
                        .document_pages
                        .get(&document_id)
                        .unwrap()
                        .iter()
                        .map(|page_id| {
                            let page = state.entities.pages_by_id.get(page_id).unwrap();
                            div(bump)
                                .attr(
                                    "id",
                                    bumpalo::collections::String::from_str_in(
                                        format!("{}", page.id).as_str(),
                                        bump,
                                    )
                                    .into_bump_str(),
                                )
                                .attr("class", "page__container")
                                .attr(
                                    "style",
                                    bumpalo::collections::String::from_str_in(
                                        format!("padding-top: {}%", 100.0 * page.aspect_ratio)
                                            .as_str(),
                                        bump,
                                    )
                                    .into_bump_str(),
                                )
                                .children(
                                    bumpalo::collections::Vec::from_iter_in(
                                        state.entities.page_anchors.get(page_id).unwrap_or(&HashSet::new()).iter().map(|anchor_id| {
                                            let anchor = state.entities.anchors_by_id.get(anchor_id).unwrap();

                                            if anchor.title == "" {
                                                div(bump)
                                                    .attr("style", bumpalo::collections::String::from_str_in(
                                                        format!(
                                                            "top: {}%; left: {}%;",
                                                            anchor.position_top,
                                                            anchor.position_left
                                                        ).as_str(),
                                                        bump
                                                    ).into_bump_str())
                                                    // .on("click", move |_root, _vdom, event| {
                                                        
                                                    // })
                                                    .attr("id", bumpalo::collections::String::from_str_in(
                                                        format!("{}", anchor_id).as_str(),
                                                        bump
                                                    ).into_bump_str())
                                                    .child(
                                                        svg(bump)
                                                            .attr("width", "100")
                                                            .attr("height", "100")
                                                            .child(
                                                                path(bump)
                                                                    .attr("d", "M10,10 L90,10 L90,90")
                                                                    .finish()
                                                            )
                                                            .finish()
                                                        )
                                                    .finish()
                                            } else {
                                                a(bump)
                                                    .attr("style", bumpalo::collections::String::from_str_in(
                                                        format!(
                                                            "top: {}%; left: {}%;",
                                                            anchor.position_top,
                                                            anchor.position_left
                                                        ).as_str(),
                                                        bump
                                                    ).into_bump_str())
                                                    // .on("click", move |_root, _vdom, event| {
                                                        
                                                    // })
                                                    .attr("id", bumpalo::collections::String::from_str_in(
                                                        format!("{}", anchor_id).as_str(),
                                                        bump
                                                    ).into_bump_str())
                                                    .child(text(bumpalo::collections::String::from_str_in(
                                                        format!("{}", anchor.title).as_str(),
                                                        bump
                                                    ).into_bump_str()))
                                                    .finish()
                                            }
                                        }).chain(
                                            state.entities.page_user_anchors.get(page_id).unwrap_or(&HashSet::new()).iter().map(|anchor_id| {
                                                let anchor = state.entities.user_anchors_by_id.get(anchor_id).unwrap();
    
                                                if anchor.title == "" {
                                                    div(bump)
                                                        .attr("class", "user_anchor")
                                                        .attr("style", bumpalo::collections::String::from_str_in(
                                                            format!(
                                                                "top: {}%; left: {}%;",
                                                                anchor.position_top,
                                                                anchor.position_left
                                                            ).as_str(),
                                                            bump
                                                        ).into_bump_str())
                                                        // .on("click", move |_root, _vdom, event| {
                                                            
                                                        // })
                                                        .attr("id", bumpalo::collections::String::from_str_in(
                                                            format!("{}", anchor_id).as_str(),
                                                            bump
                                                        ).into_bump_str())
                                                        .child(
                                                            svg(bump)
                                                                .attr("width", "100")
                                                                .attr("height", "100")
                                                                .child(
                                                                    path(bump)
                                                                        .attr("d", "M10,10 L90,10 L90,90")
                                                                        .finish()
                                                                )
                                                                .finish()
                                                            )
                                                        .finish()
                                                } else {
                                                    a(bump)
                                                        .attr("class", "user_anchor")
                                                        .attr("style", bumpalo::collections::String::from_str_in(
                                                            format!(
                                                                "top: {}%; left: {}%;",
                                                                anchor.position_top,
                                                                anchor.position_left
                                                            ).as_str(),
                                                            bump
                                                        ).into_bump_str())
                                                        // .on("click", move |_root, _vdom, event| {
                                                            
                                                        // })
                                                        .attr("id", bumpalo::collections::String::from_str_in(
                                                            format!("{}", anchor_id).as_str(),
                                                            bump
                                                        ).into_bump_str())
                                                        .child(text(bumpalo::collections::String::from_str_in(
                                                            format!("{}", anchor.title).as_str(),
                                                            bump
                                                        ).into_bump_str()))
                                                        .finish()
                                                }
                                            })
                                        ).chain(vec![
                                            img(bump)
                                        .attr("class", "page__image")
                                        .attr(
                                            "src",
                                            if (state.ui.course_screen.relative_scroll - page.height).abs() < 3.0 * page.aspect_ratio {
                                            bumpalo::collections::String::from_str_in(
                                                    format!(
                                                        "https://synchrotron.nsenger.com/{}",
                                                        page.image_path
                                                    ).as_str(),
                                                    bump,
                                                )
                                                .into_bump_str()
                                            } else {
                                                bumpalo::collections::String::from_str_in("data:image/gif;base64,R0lGODlhAQABAIAAAP///wAAACwAAAAAAQABAAACAkQBADs=", bump).into_bump_str()
                                            }
                                        )
                                        .finish()
                                        ]),
                                        bump
                                    )
                                )
                                .finish()
                        }),
                    bump,
                ))
                .finish()],
            bump,
        ))
        .finish()
}
