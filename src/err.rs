use std::sync::mpsc::SendError;
use ggez::GameError;
use std::convert::From;

#[derive(Debug)]
pub struct GgError(pub String);

impl From<GameError> for GgError {
    fn from(error: GameError) -> Self {
        GgError(error.to_string())
    }
}

impl From<GgError> for GameError {
    fn from(error: GgError) -> Self {
        ggez::GameError::FilesystemError(error.0) // TODO change?
    }
}

impl From<&str> for GgError {
    fn from(error: &str) -> Self {
        GgError(error.to_string())
    }
}

impl<T> From<SendError<T>> for GgError {
    fn from(error: SendError<T>) -> Self {
        GgError(error.to_string())
    }
}

impl From<std::io::Error> for GgError {
    fn from(error: std::io::Error) -> Self {
        GgError(error.to_string())
    }
}

impl From<serde_cbor::error::Error> for GgError {
    fn from(error: serde_cbor::error::Error) -> Self {
        GgError(error.to_string())
    }
}

impl From<std::sync::mpsc::RecvError> for GgError {
    fn from(error: std::sync::mpsc::RecvError) -> Self {
        GgError(error.to_string())
    }
}

impl From<recs::NotFound> for GgError {
    fn from(_: recs::NotFound) -> Self {
        GgError("Entity not found".to_string())
    }
}

pub type GgResult<T = ()> = Result<T, GgError>;