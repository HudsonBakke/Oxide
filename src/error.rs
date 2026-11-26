use std::{error::Error, fmt};

/// Error returned by public Oxide operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OxideError {
    InvalidArgument {
        /// Name of the invalid argument.
        argument: &'static str,
        /// Diagnostic explanation of the failure.
        message: &'static str,
    },

    InvalidOperation {
        /// Diagnostic explanation of the failure.
        message: &'static str,
    },
}

/// Result of a core operation, containing OxideError on failure.
pub type OxideResult<T> = Result<T, OxideError>;

impl fmt::Display for OxideError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArgument { argument, message } => {
                write!(f, "invalid argument '{}': {}", argument, message,)
            }

            Self::InvalidOperation { message } => {
                write!(f, "invalid operation: {}", message,)
            }
        }
    }
}

impl Error for OxideError {}
