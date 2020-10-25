use std::{cell::RefCell, rc::Rc};

use fluorophore::{button, text_input, ButtonType};
use futures::{future::ready, StreamExt};
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

#[derive(Clone)]
enum Msg {
    Increment,
    Decrement,
    Double,
}

#[topo::nested]
fn root() -> Div {
    let (ct, dispatch) = mox_stream(
        Rc::new(RefCell::new(0)),
        |state, msg| match msg {
            Msg::Increment => {
                *(state.borrow_mut()) += 1;
                state.clone()
            }
            Msg::Decrement => {
                *(state.borrow_mut()) -= 1;
                state.clone()
            }
            Msg::Double => {
                *(state.borrow_mut()) *= 2;
                state.clone()
            }
        },
        |stream| {
            stream
                .filter(|msg| match msg {
                    Msg::Increment => ready(true),
                    _ => ready(false),
                })
                .map(|_| Msg::Double)
        },
    );
    let (input_value, set_input_value) = state(|| "".to_owned());

    let mut root = div();

    root = root.child(
        mox! { <div>{% "hello world from moxie! ({}) ({})", ct.borrow(), input_value }</div> },
    );
    root = root.child(mox! {
        <button button_type={ButtonType::CTA} on_click={move |_| {dispatch(Msg::Increment);}} text="Increment" />
    });
    root = root.child(mox! {
        <text_input
            oninput={move |ev| {
                let event: &sys::Event = ev.as_ref();
                let target = event.target().unwrap();
                let input: sys::HtmlInputElement = target.dyn_into().unwrap();
                let val = input.value();
                set_input_value.set(val);
            }}
            value={format!("{}", input_value)}
            placeholder="bar"
            label="quxor"
        />
    });

    root.build()
}
