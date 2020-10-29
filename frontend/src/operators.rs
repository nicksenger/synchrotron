use std::rc::Rc;

use futures::{future::ready, FutureExt, Stream, StreamExt};

use crate::{
    operations::{login, register},
    Msg, Route,
};

pub fn login_operator(in_stream: impl Stream<Item = Rc<Msg>>) -> impl Stream<Item = Msg> {
    in_stream
        .filter_map(|msg| match &*msg {
            Msg::Login(payload) => ready(Some(payload.clone())),
            _ => ready(None),
        })
        .flat_map(|payload| login(payload).into_stream())
}

pub fn register_operator(in_stream: impl Stream<Item = Rc<Msg>>) -> impl Stream<Item = Msg> {
    in_stream
        .filter_map(|msg| match &*msg {
            Msg::Register(payload) => ready(Some(payload.clone())),
            _ => ready(None),
        })
        .flat_map(|payload| register(payload).into_stream())
}

pub fn navigation_operator(in_stream: impl Stream<Item = Rc<Msg>>) -> impl Stream<Item = Msg> {
    in_stream.filter_map(|msg| match &*msg {
        Msg::LoginSuccess(_) => ready(Some(Msg::Navigate(Route::Welcome))),
        Msg::RegisterSuccess(_) => ready(Some(Msg::Navigate(Route::Login))),
        Msg::Logout => ready(Some(Msg::Navigate(Route::Login))),
        _ => ready(None),
    })
}
