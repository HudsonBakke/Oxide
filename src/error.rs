use std::{
    error::Error,
    fmt,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OxideError {
    InvalidArgument {
        argument: &'static str,
        message: &'static str,
    },

    InvalidOperation {
        message: &'static str,
    },
}

pub type OxideResult<T> =
    Result<T, OxideError>;

impl fmt::Display for OxideError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidArgument {
                argument,
                message,
            } => {
                write!(
                    f,
                    "invalid argument '{}': {}",
                    argument,
                    message,
                )
            }

            Self::InvalidOperation {
                message,
            } => {
                write!(
                    f,
                    "invalid operation: {}",
                    message,
                )
            }
        }
    }
}

impl Error for OxideError {}