use crate::messages::Msg;

pub mod course_screen;
mod courses_screen;
mod login_screen;
mod register_screen;

pub struct Model {
    pub course_screen: course_screen::Model,
    pub courses_screen: courses_screen::Model,
    pub login_screen: login_screen::Model,
    pub register_screen: register_screen::Model,
}

impl Model {
    pub fn new() -> Self {
        Self {
            course_screen: course_screen::Model::new(),
            courses_screen: courses_screen::Model::new(),
            login_screen: login_screen::Model::default(),
            register_screen: register_screen::Model::default(),
        }
    }

    pub fn update(&mut self, message: &Msg) {
        self.course_screen.update(message);
        self.courses_screen.update(message);
        self.login_screen.update(message);
        self.register_screen.update(message);
    }
}
