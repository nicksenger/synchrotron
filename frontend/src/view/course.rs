use std::collections::HashSet;

use iced_web::{dodrio, dodrio::bumpalo, Bus};
use wasm_bindgen::JsCast;
use web_sys::console;

use crate::{
    messages::{ui, Msg},
    state::{Model, ui::course_screen::CursorMode},
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

    let current_mode = state.ui.course_screen.mode.clone();
    let hotkey_bus = bus.clone();
    let scroll_bus = bus.clone();
    let track_bus = bus.clone();
    let bookmark_bus = bus.clone();

    div::<'b>(bump)
        .attr("tabindex", "0")
        .attr("autofocus", "true")
        .attr("class", "synchrotron__inner")
        .on("keydown", move |_root, _vdom, event| {
            let ev = event.dyn_into::<web_sys::KeyboardEvent>().ok().unwrap();
            if ev.shift_key() {
                match ev.key().as_str() {
                    "A" => {
                        hotkey_bus.publish(Msg::Ui(ui::Msg::Course(
                            ui::course::Msg::ToggleMode(CursorMode::Add),
                        )));
                    }
                    "M" => {
                        hotkey_bus.publish(Msg::Ui(ui::Msg::Course(
                            ui::course::Msg::ToggleMode(CursorMode::Move),
                        )));
                    },
                    "R" => {
                        hotkey_bus.publish(Msg::Ui(ui::Msg::Course(
                            ui::course::Msg::ToggleMode(CursorMode::Delete),
                        )));
                    },
                    "U" => {
                        hotkey_bus.publish(Msg::Ui(ui::Msg::Course(
                            ui::course::Msg::ToggleMode(CursorMode::Upgrade),
                        )));
                    },
                    "P" => {
                        hotkey_bus.publish(Msg::Ui(ui::Msg::Course(
                            ui::course::Msg::TogglePlayback,
                        )));
                    },
                    _ => {},
                }
            }
            
        })
        .children(bumpalo::collections::Vec::from_iter_in(
            vec![div(bump)
                .attr("id", "page-container")
                .attr("class", "synchrotron__page-container")
                .attr(
                    "style",
                    bumpalo::collections::String::from_str_in(
                        format!("cursor: {}", match state.ui.course_screen.mode {
                            CursorMode::Add => "pointer",
                            CursorMode::Delete => "no-drop",
                            CursorMode::Move => "grab",
                            CursorMode::Upgrade => "copy",
                            _ => "auto"
                        })
                            .as_str(),
                        bump,
                    )
                    .into_bump_str()
                )
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
                                            let a_id = anchor_id.clone();

                                            let anchor_bus = bus.clone();
                                            let text_anchor_bus = bus.clone();

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
                                                    .on("click", move |_root, _vdom, event| {
                                                        anchor_bus.publish(Msg::Ui(ui::Msg::Course(
                                                            ui::course::Msg::SelectAnchor(
                                                                a_id,
                                                            ),
                                                        )));
                                                    })
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
                                                    .on("click", move |_root, _vdom, event| {
                                                        text_anchor_bus.publish(Msg::Ui(ui::Msg::Course(
                                                            ui::course::Msg::SelectAnchor(
                                                                a_id,
                                                            ),
                                                        )));
                                                    })
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
                                                let a_id = anchor_id.clone();

                                                let user_anchor_bus = bus.clone();
                                                let text_user_anchor_bus = bus.clone();
    
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
                                                        .on("click", move |_root, _vdom, event| {
                                                            user_anchor_bus.publish(Msg::Ui(ui::Msg::Course(
                                                                ui::course::Msg::SelectUserAnchor(
                                                                    a_id,
                                                                ),
                                                            )));
                                                        })
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
                                                        .on("click", move |_root, _vdom, event| {
                                                            text_user_anchor_bus.publish(Msg::Ui(ui::Msg::Course(
                                                                ui::course::Msg::SelectUserAnchor(
                                                                    a_id,
                                                                ),
                                                            )));
                                                        })
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
                .finish(),
                div(bump)
                    .attr("class", "synchrotron__menu")
                    .children(
                        bumpalo::collections::Vec::from_iter_in(
                            vec![
                                select(bump)
                                    .attr("class", "synchrotron__dropdown")
                                    .on("input", move |_root, _vdom, event| {
                                        let el = match event
                                            .target()
                                            .and_then(|t| t.dyn_into::<web_sys::HtmlSelectElement>().ok())
                                        {
                                            None => return,
                                            Some(el) => el,
                                        };
                    
                                        track_bus.publish(Msg::Ui(ui::Msg::Course(
                                            ui::course::Msg::SelectTrack(
                                                el.value().parse::<i32>().unwrap(),
                                            ),
                                        )));
                                    })
                                    .children(
                                        bumpalo::collections::Vec::from_iter_in(
                                            state.entities.document_tracks.get(&document_id).unwrap().iter().map(|track_id| {
                                                let track = state.entities.tracks_by_id.get(track_id).unwrap();
                                                option(bump)
                                                    .attr("value", bumpalo::collections::String::from_str_in(format!("{}", track_id).as_str(), bump).into_bump_str())
                                                    .attr("title", bumpalo::collections::String::from_str_in(format!("{}", track.title).as_str(), bump).into_bump_str())
                                                    .child(text(bumpalo::collections::String::from_str_in(format!("{}", track.title).as_str(), bump).into_bump_str()))
                                                    .finish()
                                            }),
                                            bump
                                        )
                                    )
                                    .finish(),
                                audio(bump)
                                    .attr("id", "audio")
                                    .attr("controls", "true")
                                    .attr("class", "synchrotron__audio")
                                    .finish(),
                                select(bump)
                                    .attr("class", "synchrotron__dropdown")
                                    .on("input", move |_root, _vdom, event| {
                                        let el = match event
                                            .target()
                                            .and_then(|t| t.dyn_into::<web_sys::HtmlSelectElement>().ok())
                                        {
                                            None => return,
                                            Some(el) => el,
                                        };
                    
                                        bookmark_bus.publish(Msg::Ui(ui::Msg::Course(
                                            ui::course::Msg::SelectBookmark(
                                                el.value().parse::<i32>().unwrap(),
                                            ),
                                        )));
                                    })
                                    .children(
                                        bumpalo::collections::Vec::from_iter_in(
                                            state.entities.document_bookmarks.get(&document_id).unwrap().iter().map(|bookmark_id| {
                                                let bookmark = state.entities.bookmarks_by_id.get(bookmark_id).unwrap();
                                                option(bump)
                                                    .attr("value", bumpalo::collections::String::from_str_in(format!("{}", bookmark_id).as_str(), bump).into_bump_str())
                                                    .attr("title", bumpalo::collections::String::from_str_in(format!("{}", bookmark.title).as_str(), bump).into_bump_str())
                                                    .child(text(bumpalo::collections::String::from_str_in(format!("{}", bookmark.title).as_str(), bump).into_bump_str()))
                                                    .finish()
                                            }),
                                            bump
                                        )
                                    )
                                    .finish()
                            ],
                            bump
                        ),
                    )
                    .finish()
            ],
            bump,
        ))
        .finish()
}
