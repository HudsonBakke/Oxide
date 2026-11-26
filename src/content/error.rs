use std::{fmt, io, path::PathBuf};

/// Error produced while validating, importing, caching, or loading content.
#[derive(Debug)]
pub enum ContentError {
    FileNotFound {
        /// Source path associated with the failure.
        path: PathBuf,
    },

    UnsupportedFormat {
        /// Source path associated with the failure.
        path: PathBuf,
        /// Detected extension, or None when the path has none.
        extension: Option<String>,
    },

    UnsupportedAssetType {
        /// Source path associated with the failure.
        path: PathBuf,
        /// Name of the requested engine asset type.
        asset_type: &'static str,
        /// Detected extension, or None when the path has none.
        extension: Option<String>,
    },

    ImportFailed {
        /// Source path associated with the failure.
        path: PathBuf,
        /// Diagnostic explanation of the failure.
        message: String,
    },

    /// An underlying filesystem or stream operation failed.
    Io(io::Error),

    /// A compiled cache is corrupt, incompatible, or stale; normal loads fall back to the source.
    InvalidCache(String),

    SubObjectNotSupported {
        /// Source path associated with the failure.
        path: PathBuf,
        /// Name of the requested engine asset type.
        asset_type: &'static str,
        /// Requested named subobject.
        subobject: String,
    },

    SubObjectNotFound {
        /// Source path associated with the failure.
        path: PathBuf,
        /// Name of the requested engine asset type.
        asset_type: &'static str,
        /// Requested named subobject.
        subobject: String,
    },

    AmbiguousSubObject {
        /// Source path associated with the failure.
        path: PathBuf,
        /// Name of the requested engine asset type.
        asset_type: &'static str,
    },

    InvalidSubObject {
        /// Diagnostic explanation of the failure.
        message: &'static str,
    },
}

/// Result of an asset operation, containing ContentError on failure.
pub type ContentResult<T> = Result<T, ContentError>;

impl fmt::Display for ContentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileNotFound { path } => {
                write!(f, "content file not found: {}", path.display())
            }

            Self::UnsupportedFormat { path, extension } => {
                write!(
                    f,
                    "unsupported content format '{}' for {}",
                    extension.as_deref().unwrap_or("<none>"),
                    path.display()
                )
            }

            Self::UnsupportedAssetType {
                path,
                asset_type,
                extension,
            } => {
                write!(
                    f,
                    "content format '{}' cannot be loaded as {}: {}",
                    extension.as_deref().unwrap_or("<none>"),
                    asset_type,
                    path.display()
                )
            }

            Self::ImportFailed { path, message } => {
                write!(f, "failed to import {}: {}", path.display(), message)
            }

            Self::Io(error) => {
                write!(f, "content I/O error: {}", error)
            }

            Self::InvalidCache(message) => {
                write!(f, "invalid compiled Oxide asset: {}", message)
            }

            Self::SubObjectNotSupported {
                path,
                asset_type,
                subobject,
            } => {
                write!(
                    f,
                    "{} does not support subobject '{}' when loading {}",
                    path.display(),
                    subobject,
                    asset_type,
                )
            }

            Self::SubObjectNotFound {
                path,
                asset_type,
                subobject,
            } => {
                write!(
                    f,
                    "subobject '{}' was not found when loading {} from {}",
                    subobject,
                    asset_type,
                    path.display(),
                )
            }

            Self::AmbiguousSubObject { path, asset_type } => {
                write!(
                    f,
                    "{} contains multiple objects that can be loaded as {}; specify a subobject",
                    path.display(),
                    asset_type,
                )
            }

            Self::InvalidSubObject { message } => {
                write!(f, "invalid content subobject: {}", message,)
            }
        }
    }
}

impl std::error::Error for ContentError {}

impl From<io::Error> for ContentError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}
