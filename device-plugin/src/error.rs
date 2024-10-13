#[derive(Debug)]
pub enum Error {
    GrpcStatus(tonic::Status),
    Listen(std::io::Error),
    Socket(std::io::Error),
    Transport(tonic::transport::Error),
}

impl From<tonic::Status> for Error {
    fn from(value: tonic::Status) -> Self {
        Error::GrpcStatus(value)
    }
}

impl From<tonic::transport::Error> for Error {
    fn from(value: tonic::transport::Error) -> Self {
        Error::Transport(value)
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::GrpcStatus(v) => v.fmt(f),
            Error::Listen(v) => v.fmt(f),
            Error::Socket(v) => v.fmt(f),
            Error::Transport(v) => v.fmt(f),
        }
    }
}

impl Error {
    pub fn listen(value: std::io::Error) -> Self {
        Error::Listen(value)
    }

    pub fn socket(value: std::io::Error) -> Self {
        Error::Socket(value)
    }
}
