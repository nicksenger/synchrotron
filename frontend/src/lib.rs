use std::{cell::RefCell, rc::Rc};

use fluorophore::{button, text_input, ButtonType};
use futures::{channel::mpsc, future::ready, stream::Stream, StreamExt, FutureExt};
use mox::mox;
use moxie::state;
use moxie_dom::{
    elements::text_content::{div, Div},
    prelude::*,
};
use moxie_streams::mox_stream;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(start)]
pub fn begin() {
    moxie_dom::boot(document().body(), root);
}

struct User {
    username: String,
    token: String
}

#[derive(Clone)]
enum Route {
    Login,
    Register,
    Welcome,
}

#[derive(Clone)]
struct LoginPayload {
    username: String,
    password: String,
}

#[derive(Clone)]
struct LoginSuccessPayload {
    username: String,
    token: String,
}

#[derive(Clone)]
struct LoginFailedPayload {
    message: String,
}

#[derive(Clone)]
struct RegisterPayload {
    username: String,
    password: String,
}

#[derive(Clone)]
struct RegisterSuccessPayload {
    user_id: i32,
}

#[derive(Clone)]
struct RegisterFailedPayload {
    message: String,
}

struct AppState {
    route: Route,
    loading: bool,
    user: Option<User>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            route: Route::Login,
            loading: false,
            user: None,
        }
    }
}

#[derive(Clone)]
enum Msg {
    Login(LoginPayload),
    LoginSuccess(LoginSuccessPayload),
    LoginFailed(LoginFailedPayload),
    Register(RegisterPayload),
    RegisterSuccess(RegisterSuccessPayload),
    RegisterFailed(RegisterFailedPayload),
    Navigate(Route),
    Logout
}

fn reducer(state: &Rc<RefCell<AppState>>, action: Msg) -> Rc<RefCell<AppState>> {
    let new_state = state.clone();
    match action {
        Msg::Login(_) => {
            (*new_state.borrow_mut()).loading = true;
        }

        Msg::LoginSuccess(payload) => {
            (*new_state.borrow_mut()).loading = false;
            (*new_state.borrow_mut()).user = Some(User {
                username: payload.username,
                token: payload.token
            });
        }

        Msg::LoginFailed(_) => {
            (*new_state.borrow_mut()).loading = false;
            (*new_state.borrow_mut()).user = None;
        }

        Msg::Register(_) => {
            (*new_state.borrow_mut()).loading = true;
        }

        Msg::RegisterSuccess(_) => {
            (*new_state.borrow_mut()).loading = false;
        }

        Msg::RegisterFailed(_) => {
            (*new_state.borrow_mut()).loading = false;
        }

        Msg::Navigate(route) => {
            (*new_state.borrow_mut()).route = route;
        }

        Msg::Logout => {
            (*new_state.borrow_mut()).route = Route::Login;
        }
    }
    new_state
}

#[topo::nested]
fn root() -> Div {
    let (app_state, dispatch) = mox_stream(
        Rc::new(RefCell::new(AppState::default())),
        reducer,
        |stream| {
            stream
                .filter(|msg| match msg {
                    Msg::Login(_) | Msg::Register(_) => ready(true),
                    _ => ready(false),
                })
                .flat_map(|msg| match msg {
                    Msg::Login(payload) => {
                        ready(Msg::Logout).into_stream()
                    }
                    Msg::Register(payload) => {
                        ready(Msg::Logout).into_stream()
                    }
                    _ => {
                        ready(Msg::Logout).into_stream()
                    }
                })
        },
    );

    let mut root = div();

    // root = root.child(
    //     mox! { <div>{% "hello world from moxie! ({}) ({})", ct.borrow(), input_value }</div> },
    // );
    // root = root.child(mox! {
    //     <button button_type={ButtonType::CTA} on_click={move |_| {dispatch(Msg::Increment);}} text="Increment" />
    // });
    // root = root.child(mox! {
    //     <text_input
    //         oninput={move |ev| {
    //             let event: &sys::Event = ev.as_ref();
    //             let target = event.target().unwrap();
    //             let input: sys::HtmlInputElement = target.dyn_into().unwrap();
    //             let val = input.value();
    //             set_input_value.set(val);
    //         }}
    //         value={format!("{}", input_value)}
    //         placeholder="bar"
    //         label="quxor"
    //     />
    // });

    root.build()
}
