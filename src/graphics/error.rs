use std::{
    error::Error,
    fmt,
    io,
};

#[derive(Debug)]
pub enum GraphicsError {
    Windows(windows::core::Error),
    Io(io::Error),
}

impl fmt::Display for GraphicsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Windows(error) => {
                write!(f, "graphics API error: {error}")
            }

            Self::Io(error) => {
                write!(f, "graphics I/O error: {error}")
            }
        }
    }
}

impl Error for GraphicsError {}

impl From<windows::core::Error> for GraphicsError {
    fn from(error: windows::core::Error) -> Self {
        Self::Windows(error)
    }
}

impl From<io::Error> for GraphicsError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

pub type GraphicsResult<T> = Result<T, GraphicsError>;