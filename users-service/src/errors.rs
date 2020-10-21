use std::{error, fmt};

#[derive(Debug)]
pub enum UsersServiceError {
    Database(sqlx::Error),
    Decryption(bcrypt::BcryptError),
}

impl fmt::Display for UsersServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Database(ref err) => write!(f, "Database error: {}", err),
            Self::Decryption(ref err) => write!(f, "Decryption error: {}", err),
        }
    }
}

impl error::Error for UsersServiceError {
    fn cause(&self) -> Option<&(dyn error::Error)> {
        match *self {
            Self::Database(ref err) => Some(err),
            Self::Decryption(ref err) => Some(err),
        }
    }
}

impl From<sqlx::Error> for UsersServiceError {
    fn from(err: sqlx::Error) -> UsersServiceError {
        UsersServiceError::Database(err)
    }
}

impl From<bcrypt::BcryptError> for UsersServiceError {
    fn from(err: bcrypt::BcryptError) -> UsersServiceError {
        UsersServiceError::Decryption(err)
    }
}

impl From<UsersServiceError> for tonic::Status {
    fn from(err: UsersServiceError) -> tonic::Status {
        tonic::Status::unknown(err.to_string())
    }
}
