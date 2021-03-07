use std::rc::Rc;

use iced_futures::futures::{
    channel::mpsc,
    future::ready,
    stream::{select, select_all},
    Stream, StreamExt,
};

use crate::messages::Msg;

pub fn root_effect(in_stream: impl Stream<Item = Rc<Msg>>) -> impl Stream<Item = Msg> {
    combine_effects(in_stream, vec![Box::new(logger)])
}

pub fn combine_effects<InStream, OutStream>(
    in_stream: InStream,
    effects: Vec<Box<dyn FnOnce(mpsc::UnboundedReceiver<Rc<Msg>>) -> OutStream>>,
) -> impl Stream<Item = Msg>
where
    InStream: Stream<Item = Rc<Msg>>,
    OutStream: Stream<Item = Msg> + Unpin,
{
    let mut senders = vec![];
    let mut receivers = vec![];

    for effect in effects {
        let (sender, receiver) = mpsc::unbounded();
        senders.push(sender);
        receivers.push(effect(receiver));
    }

    let s = in_stream
        .map(move |msg| {
            for sender in senders.iter_mut() {
                sender.unbounded_send(msg.clone());
            }
            Msg::Noop
        })
        .filter(|_| ready(false));

    select(
        s,
        select_all(receivers),
    )
}

pub fn logger(in_stream: impl Stream<Item = Rc<Msg>>) -> impl Stream<Item = Msg> {
    in_stream
        .map(|m| {
            unsafe {
                web_sys::console::log_1(&wasm_bindgen::JsValue::from_str("got a message!"));
            }
            Msg::Noop
        })
        .filter(|_| ready(false))
}
