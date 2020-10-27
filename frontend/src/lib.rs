#![type_length_limit = "1382035"]

use std::{cell::RefCell, rc::Rc};

use fluorophore::{button, text_input, ButtonType};
use futures::StreamExt;
use mox::mox;
use moxie::{load_once, once, state};
use moxie_dom::{
    elements::text_content::{div, Div},
    prelude::*,
};
use moxie_streams::{combine_operators, mox_stream};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod operations;
mod operators;

use operators::{login_operator, navigation_operator, register_operator};

#[wasm_bindgen(start)]
pub fn begin() {
    moxie_dom::boot(document().body(), root);
}

pub struct User {
    pub username: String,
    pub token: String,
}

#[derive(Clone)]
pub enum Route {
    Login,
    Register,
    Welcome,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LoginSuccessPayload {
    pub username: String,
    pub token: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LoginFailedPayload {
    pub message: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegisterPayload {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegisterSuccessPayload {
    pub user_id: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegisterFailedPayload {
    pub message: String,
}

pub struct AppState {
    pub route: Route,
    pub loading: bool,
    pub error_message: Option<String>,
    pub user: Option<User>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            route: Route::Login,
            loading: false,
            error_message: None,
            user: None,
        }
    }
}

#[derive(Clone)]
pub enum Msg {
    Login(LoginPayload),
    LoginSuccess(LoginSuccessPayload),
    LoginFailed(LoginFailedPayload),
    Register(RegisterPayload),
    RegisterSuccess(RegisterSuccessPayload),
    RegisterFailed(RegisterFailedPayload),
    Navigate(Route),
    Logout,
}

fn reducer(state: &Rc<RefCell<AppState>>, msg: Rc<Msg>) -> Rc<RefCell<AppState>> {
    let new_state = state.clone();
    match *msg {
        Msg::Login(_) => {
            (*new_state.borrow_mut()).loading = true;
        }

        Msg::LoginSuccess(payload) => {
            (*new_state.borrow_mut()).loading = false;
            (*new_state.borrow_mut()).user = Some(User {
                username: payload.username,
                token: payload.token,
            });
        }

        Msg::LoginFailed(payload) => {
            (*new_state.borrow_mut()).error_message = Some(payload.message);
            (*new_state.borrow_mut()).loading = false;
            (*new_state.borrow_mut()).user = None;
        }

        Msg::Register(_) => {
            (*new_state.borrow_mut()).loading = true;
        }

        Msg::RegisterSuccess(_) => {
            (*new_state.borrow_mut()).loading = false;
        }

        Msg::RegisterFailed(payload) => {
            (*new_state.borrow_mut()).error_message = Some(payload.message);
            (*new_state.borrow_mut()).loading = false;
        }

        Msg::Navigate(route) => {
            (*new_state.borrow_mut()).route = route;
        }

        Msg::Logout => {
            (*new_state.borrow_mut()).user = None;
        }
    }
    new_state
}

#[topo::nested]
fn root() -> Div {
    let (app_state, dispatch) =
        mox_stream(Rc::new(RefCell::new(AppState::default())), reducer, || {
            Box::new(combine_operators!(
                login_operator,
                register_operator,
                navigation_operator
            ))
        });
    let (d1, d2, d3, d4, d5) = once(|| {
        let d = Rc::new(dispatch);
        (d.clone(), d.clone(), d.clone(), d.clone(), d.clone())
    });

    let mut root = div();

    if app_state.borrow().loading {
        root = root.child(mox! { <div>{% "Loading..." }</div> });
    } else {
        if let Some(message) = &app_state.borrow().error_message {
            root = root.child(mox! { <div>{% "ERROR: {}", message }</div> });
        }
        match app_state.borrow().route {
            Route::Login => {
                let (username, set_username) = state(|| "".to_owned());
                let (password, set_password) = state(|| "".to_owned());
                root = root.child(mox! { <div>{% "Login:" }</div> });
                root = root.child(mox! {
                    <text_input
                        oninput={move |ev| {
                            let event: &sys::Event = ev.as_ref();
                            let target = event.target().unwrap();
                            let input: sys::HtmlInputElement = target.dyn_into().unwrap();
                            let val = input.value();
                            set_username.set(val);
                        }}
                        value={format!("{}", username)}
                        placeholder="Enter your username here"
                        label="Username:"
                    />
                });
                root = root.child(mox! {
                    <text_input
                        input_type="password"
                        oninput={move |ev| {
                            let event: &sys::Event = ev.as_ref();
                            let target = event.target().unwrap();
                            let input: sys::HtmlInputElement = target.dyn_into().unwrap();
                            let val = input.value();
                            set_password.set(val);
                        }}
                        value={format!("{}", password)}
                        placeholder="Enter your password here"
                        label="Password:"
                    />
                });
                root = root.child(mox! {
                    <button
                        button_type={ButtonType::CTA}
                        on_click={move |_| {d1(Msg::Login(LoginPayload {
                            username: username.to_string(),
                            password: password.to_string()
                        }));}}
                        text="Login"
                    />
                });
                root = root.child(mox! {
                    <button
                        button_type={ButtonType::Secondary} on_click={move |_| {d2(Msg::Navigate(Route::Register));}}
                        text="Register"
                    />
                });
            }

            Route::Register => {
                let (username, set_username) = state(|| "".to_owned());
                let (password, set_password) = state(|| "".to_owned());
                root = root.child(mox! { <div>{% "Register:" }</div> });
                root = root.child(mox! {
                    <text_input
                        oninput={move |ev| {
                            let event: &sys::Event = ev.as_ref();
                            let target = event.target().unwrap();
                            let input: sys::HtmlInputElement = target.dyn_into().unwrap();
                            let val = input.value();
                            set_username.set(val);
                        }}
                        value={format!("{}", username)}
                        placeholder="Enter your username here"
                        label="Username:"
                    />
                });
                root = root.child(mox! {
                    <text_input
                        input_type="password"
                        oninput={move |ev| {
                            let event: &sys::Event = ev.as_ref();
                            let target = event.target().unwrap();
                            let input: sys::HtmlInputElement = target.dyn_into().unwrap();
                            let val = input.value();
                            set_password.set(val);
                        }}
                        value={format!("{}", password)}
                        placeholder="Enter your password here"
                        label="Password:"
                    />
                });
                root = root.child(mox! {
                    <button
                        button_type={ButtonType::CTA}
                        on_click={move |_| {d3(Msg::Register(RegisterPayload {
                            username: username.to_string(),
                            password: password.to_string()
                        }));}}
                        text="Register"
                    />
                });
                root = root.child(mox! {
                    <button
                        button_type={ButtonType::Secondary} on_click={move |_| {d4(Msg::Navigate(Route::Login));}}
                        text="Login"
                    />
                });
            }

            Route::Welcome => {
                root = root.child(
                    mox! { <div>{% "Welcome back, {}!", app_state.borrow().user.as_ref().map(|u| u.username.as_str()).unwrap_or("guest") }</div> },
                );
                root = root.child(mox! {
                    <button
                        button_type={ButtonType::Secondary} on_click={move |_| {d5(Msg::Logout);}}
                        text="Logout"
                    />
                });
            }
        }
    }

    root.build()
}
