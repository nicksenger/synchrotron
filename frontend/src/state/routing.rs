use crate::messages::Msg;

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
            route: Route::NotFound,
        }
    }

    pub fn update(&mut self, message: &Msg) {}
}
