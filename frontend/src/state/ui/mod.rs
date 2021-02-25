use crate::messages::Msg;

mod course_screen;
mod courses_screen;
mod login_screen;
mod register_screen;

pub struct Model {
    course_screen: course_screen::Model,
    courses_screen: courses_screen::Model,
    login_screen: login_screen::Model,
    register_screen: register_screen::Model,
}

impl Model {
    pub fn new() -> Self {
        Self {
            course_screen: course_screen::Model::new(),
            courses_screen: courses_screen::Model::new(),
            login_screen: login_screen::Model::new(),
            register_screen: register_screen::Model::new(),
        }
    }

    pub fn update(&mut self, message: &Msg) {
        self.course_screen.update(message);
        self.courses_screen.update(message);
        self.login_screen.update(message);
        self.register_screen.update(message);
    }
}
