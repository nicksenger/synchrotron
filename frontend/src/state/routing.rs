use crate::messages::{routing, Msg};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Route {
    Login,
    Register,
    Courses,
    Course(i32, Option<i32>),
    NotFound,
}

pub struct Model {
    pub route: Route,
}

impl Model {
    pub fn new(url: String) -> Self {
        Self {
            route: Route::Login,
        }
    }

    pub fn update(&mut self, message: &Msg) {
        match message {
            Msg::Routing(routing::Msg::Navigate(r)) => self.route = r.clone(),
            _ => {}
        }
    }
}
