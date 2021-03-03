use std::hash::Hash;

use iced_futures::futures::{channel::mpsc, stream::LocalBoxStream};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::state::Route;

pub fn route_change(route: Route) -> iced_web::Subscription<Route> {
    iced_web::Subscription::from_recipe(RouteChange { route })
}

pub struct RouteChange {
    route: Route,
}

impl<H, I> iced_web::subscription::Recipe<H, I> for RouteChange
where
    H: std::hash::Hasher,
{
    type Output = crate::state::Route;

    fn hash(&self, state: &mut H) {
        self.route.hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: LocalBoxStream<'static, I>,
    ) -> LocalBoxStream<'static, Self::Output> {
        let (mut sender, mut receiver) = mpsc::unbounded();

        let closure = Closure::wrap(Box::new(move |event: web_sys::PopStateEvent| {
            let pathname = web_sys::window()
                .and_then(|window| window.location().pathname().ok())
                .unwrap_or("".to_owned());
            sender.unbounded_send(Route::from(pathname));
        }) as Box<dyn FnMut(_)>);
        web_sys::window().and_then(|window| {
            window
                .add_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref())
                .ok()
        });
        closure.forget();

        Box::pin(receiver)
    }
}
