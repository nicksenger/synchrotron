use crate::messages::{routing, Msg};

impl From<String> for Route {
    fn from(x: String) -> Self {
        let mut path = x.split("/").into_iter();
        path.next();
        match path.next() {
            None | Some("") | Some("login") => Route::Login,
            Some("register") => Route::Register,
            Some("courses") => Route::Courses,
            Some("course") => match path.next() {
                Some(id) => id
                    .parse::<i32>()
                    .map(|document_id| match path.next() {
                        Some(a_id) => a_id
                            .parse::<i32>()
                            .map(|anchor_id| Route::Course(document_id, Some(anchor_id)))
                            .unwrap_or(Route::NotFound),
                        _ => Route::Course(document_id, None),
                    })
                    .unwrap_or(Route::NotFound),
                _ => Route::NotFound,
            },
            _ => Route::NotFound,
        }
    }
}

impl From<Route> for String {
    fn from(x: Route) -> Self {
        let path = match x {
            Route::Login => vec!["login".to_owned()],
            Route::Register => vec!["register".to_owned()],
            Route::Courses => vec!["courses".to_owned()],
            Route::Course(document_id, anchor_id) => {
                if let Some(id) = anchor_id {
                    vec!["course".to_owned(), document_id.to_string(), id.to_string()]
                } else {
                    vec!["course".to_owned(), document_id.to_string()]
                }
            }
            _ => vec!["".to_owned()],
        };
        format!("/{}", path.join("/"))
    }
}

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
    pub fn new(pathname: String) -> Self {
        Self {
            route: Route::from(pathname),
        }
    }

    pub fn update(&mut self, message: &Msg) {
        match message {
            Msg::Routing(routing::Msg::Navigate(r)) => self.route = r.clone(),
            _ => {}
        }
    }
}
