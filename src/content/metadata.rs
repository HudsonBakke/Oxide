use std::{
    fs,
    io::{
        Read,
        Write,
    },
    path::{
        Path,
        PathBuf,
    },
    time::UNIX_EPOCH,
};

use super::{
    format::{
        read_u32,
        read_u64,
        write_u32,
        write_u64,
    },
    ContentError,
    ContentResult,
};

#[derive(Debug, Clone)]
pub(crate) struct DependencyMetadata {
    pub path: PathBuf,
    pub size: u64,
    pub modified_nanos: u64,
}

impl DependencyMetadata {
    pub fn from_path(
        path: &Path,
    ) -> ContentResult<Self> {
        let canonical_path =
            fs::canonicalize(path)?;

        let metadata =
            fs::metadata(
                &canonical_path
            )?;

        let modified =
            metadata
                .modified()?
                .duration_since(
                    UNIX_EPOCH
                )
                .map_err(
                    |error| {
                        ContentError::InvalidCache(
                            format!(
                                "invalid source modification time: {error}"
                            )
                        )
                    }
                )?;

        let nanos =
            modified.as_nanos();

        let modified_nanos =
            u64::try_from(nanos)
                .unwrap_or(u64::MAX);

        Ok(
            Self {
                path: canonical_path,
                size: metadata.len(),
                modified_nanos,
            }
        )
    }

    pub fn is_current(
        &self,
    ) -> bool {
        let Ok(current) =
            Self::from_path(
                &self.path
            )
        else {
            return false;
        };

        current.size == self.size
            && current.modified_nanos
                == self.modified_nanos
    }
}

pub(crate) fn write_dependency(
    writer: &mut impl Write,
    dependency: &DependencyMetadata,
) -> ContentResult<()> {
    let path =
        dependency
            .path
            .to_string_lossy();

    let path_bytes =
        path.as_bytes();

    let path_length =
        u32::try_from(
            path_bytes.len()
        )
        .map_err(
            |_| {
                ContentError::InvalidCache(
                    "dependency path is too long"
                        .into()
                )
            }
        )?;

    write_u32(
        writer,
        path_length,
    )?;

    writer.write_all(
        path_bytes
    )?;

    write_u64(
        writer,
        dependency.size,
    )?;

    write_u64(
        writer,
        dependency.modified_nanos,
    )?;

    Ok(())
}

pub(crate) fn read_dependency(
    reader: &mut impl Read,
) -> ContentResult<DependencyMetadata> {
    const MAX_PATH_LENGTH: u32 =
        32 * 1024;

    let path_length =
        read_u32(reader)?;

    if path_length > MAX_PATH_LENGTH {
        return Err(
            ContentError::InvalidCache(
                "compiled dependency path is unreasonably long"
                    .into()
            )
        );
    }

    let mut path_bytes =
        vec![
            0u8;
            path_length as usize
        ];

    reader.read_exact(
        &mut path_bytes
    )?;

    let path_string =
        String::from_utf8(
            path_bytes
        )
        .map_err(
            |_| {
                ContentError::InvalidCache(
                    "compiled dependency path is not valid UTF-8"
                        .into()
                )
            }
        )?;

    let size =
        read_u64(reader)?;

    let modified_nanos =
        read_u64(reader)?;

    Ok(
        DependencyMetadata {
            path:
                PathBuf::from(
                    path_string
                ),
            size,
            modified_nanos,
        }
    )
}