use std::rc::Rc;

use iced_futures::futures::{
    channel::mpsc,
    stream::{select, LocalBoxStream},
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::{effects, messages::{Msg, routing}, state::Route};

pub fn subscribe(
    message_receiver: Option<mpsc::UnboundedReceiver<Rc<Msg>>>,
) -> iced_web::Subscription<Msg> {
    iced_web::Subscription::from_recipe(Effect { message_receiver })
}

pub struct Effect {
    message_receiver: Option<mpsc::UnboundedReceiver<Rc<Msg>>>,
}

impl<H, I> iced_web::subscription::Recipe<H, I> for Effect
where
    H: std::hash::Hasher,
{
    type Output = Msg;

    fn hash(&self, state: &mut H) {}

    fn stream(
        self: Box<Self>,
        _input: LocalBoxStream<'static, I>,
    ) -> LocalBoxStream<'static, Self::Output> {
        let (mut sender, mut receiver) = mpsc::unbounded();

        let closure = Closure::wrap(Box::new(move |event: web_sys::PopStateEvent| {
            let pathname = web_sys::window()
                .and_then(|window| window.location().pathname().ok())
                .unwrap_or("".to_owned());
            sender.unbounded_send(Msg::Routing(routing::Msg::Navigate(Route::from(pathname))));
        }) as Box<dyn FnMut(_)>);
        web_sys::window().and_then(|window| {
            window
                .add_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref())
                .ok()
        });
        closure.forget();

        Box::pin(select(
            effects::root_effect(self.message_receiver.unwrap()),
            receiver,
        ))
    }
}
