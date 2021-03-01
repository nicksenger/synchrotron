use iced_web::{
    dodrio,
    dodrio::bumpalo,
    Bus,
};
use wasm_bindgen::JsCast;

use crate::{
    messages::{authentication, routing, ui, Msg},
    state::{Model, Route},
};
use authentication::LoginRequestPayload;

pub fn render<'b, 's>(
    bump: &'b bumpalo::Bump,
    state: &'s Model,
    bus: &Bus<Msg>,
) -> dodrio::Node<'b> {
    use dodrio::builder::*;

    if state.ui.login_screen.loading {
        return p(bump).child(text(
            dodrio::bumpalo::collections::String::from_str_in(
                "Loading...",
                bump,
            )
            .into_bump_str(),
        )).finish();
    }

    let username = state.ui.login_screen.username_input_value.to_owned();
    let password = state.ui.login_screen.password_input_value.to_owned();
    let username_change_bus = bus.clone();
    let password_change_bus = bus.clone();
    let submission_bus = bus.clone();
    let link_bus = bus.clone();

    div::<'b>(bump).children(bumpalo::collections::Vec::from_iter_in(
        vec![
            p::<'b>(bump)
                .child(text(
                    dodrio::bumpalo::collections::String::from_str_in("Login", bump)
                        .into_bump_str(),
                ))
                .finish(),
            input::<'b>(bump)
                .attr(
                    "value",
                    bumpalo::collections::String::from_str_in(
                        state.ui.login_screen.username_input_value.as_str(),
                        bump,
                    )
                    .into_bump_str(),
                )
                .on("change", move |_root, _vdom, event| {
                    let text_input = match event
                        .target()
                        .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                    {
                        None => return,
                        Some(text_input) => text_input,
                    };

                    username_change_bus.publish(Msg::Ui(ui::Msg::Login(
                        ui::login::Msg::UsernameInputChanged(text_input.value()),
                    )));
                })
                .finish(),
            input::<'b>(bump)
                .attr(
                    "value",
                    bumpalo::collections::String::from_str_in(
                        state.ui.login_screen.password_input_value.as_str(),
                        bump,
                    )
                    .into_bump_str(),
                )
                .attr("type", "password")
                .on("change", move |_root, _vdom, event| {
                    let text_input = match event
                        .target()
                        .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                    {
                        None => return,
                        Some(text_input) => text_input,
                    };

                    password_change_bus.publish(Msg::Ui(ui::Msg::Login(
                        ui::login::Msg::PasswordInputChanged(text_input.value()),
                    )));
                })
                .finish(),
            button::<'b>(bump)
                .child(text(
                    dodrio::bumpalo::collections::String::from_str_in("Go", bump).into_bump_str(),
                ))
                .on("click", move |_root, _vdom, event| {
                    submission_bus.publish(Msg::Authentication(authentication::Msg::LoginRequest(
                        LoginRequestPayload {
                            username: username.clone(),
                            password: password.clone(),
                        },
                    )));
                })
                .finish(),
            div::<'b>(bump)
                .children(bumpalo::collections::Vec::from_iter_in(
                    vec![
                        text(
                            dodrio::bumpalo::collections::String::from_str_in(
                                "Don't have an account? ",
                                bump,
                            )
                            .into_bump_str(),
                        ),
                        button::<'b>(bump)
                            .child(text(
                                dodrio::bumpalo::collections::String::from_str_in("Register", bump)
                                    .into_bump_str(),
                            ))
                            .on("click", move |_root, _vdom, event| {
                                link_bus
                                    .publish(Msg::Routing(routing::Msg::Navigate(Route::Register)));
                            })
                            .finish(),
                    ],
                    bump,
                ))
                .finish(),
        ],
        bump,
    )).finish()
}
