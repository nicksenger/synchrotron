use std::{error, fmt};

#[derive(Debug)]
pub enum CoursesServiceError {
    Database(sqlx::Error),
}

impl fmt::Display for CoursesServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Database(ref err) => write!(f, "Database error: {}", err),
        }
    }
}

impl error::Error for CoursesServiceError {
    fn cause(&self) -> Option<&(dyn error::Error)> {
        match *self {
            Self::Database(ref err) => Some(err),
        }
    }
}

impl From<sqlx::Error> for CoursesServiceError {
    fn from(err: sqlx::Error) -> Self {
        Self::Database(err)
    }
}

impl From<CoursesServiceError> for tonic::Status {
    fn from(err: CoursesServiceError) -> tonic::Status {
        tonic::Status::unknown(err.to_string())
    }
}
