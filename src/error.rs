use std::error::Error;
use std::fmt;

pub type TetrisResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct TetrisError {
    pub kind: TetrisErrorKind,
    pub message: String,
}

impl TetrisError {
    pub fn new(kind: TetrisErrorKind, message: String) -> TetrisError {
        TetrisError::default()
    }
}

impl Error for TetrisError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for TetrisError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TetrisErrorKind {
    EventPump,
    VideoSubsystem,
    #[default]
    Null,
}

impl Error for TetrisErrorKind {

}

impl fmt::Display for TetrisErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub trait HandleTetrisError<T> {
}

impl<T> HandleTetrisError<T> for TetrisResult<T> {

}