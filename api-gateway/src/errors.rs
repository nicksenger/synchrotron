use std::{error, fmt};

#[derive(Debug)]
pub enum GatewayError {
    Grpc(tonic::Status),
}

impl fmt::Display for GatewayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Grpc(ref err) => write!(f, "GRPC error: {}", err),
        }
    }
}

impl error::Error for GatewayError {
    fn cause(&self) -> Option<&(dyn error::Error)> {
        match *self {
            Self::Grpc(ref err) => Some(err),
        }
    }
}

impl From<tonic::Status> for GatewayError {
    fn from(err: tonic::Status) -> GatewayError {
        GatewayError::Grpc(err)
    }
}
