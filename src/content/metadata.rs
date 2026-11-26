use std::{
    collections::HashSet,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    time::UNIX_EPOCH,
};

use super::{
    ContentError, ContentResult,
    format::{read_u32, read_u64, write_u32, write_u64},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DependencyMetadata {
    pub path: PathBuf,
    pub size: u64,
    pub modified_nanos: u64,
}

impl DependencyMetadata {
    pub fn from_path(path: &Path) -> ContentResult<Self> {
        let canonical_path = fs::canonicalize(path)?;

        let metadata = fs::metadata(&canonical_path)?;

        let modified = metadata
            .modified()?
            .duration_since(UNIX_EPOCH)
            .map_err(|error| {
                ContentError::InvalidCache(format!("invalid source modification time: {error}"))
            })?;

        let nanos = modified.as_nanos();

        let modified_nanos = u64::try_from(nanos).unwrap_or(u64::MAX);

        Ok(Self {
            path: canonical_path,
            size: metadata.len(),
            modified_nanos,
        })
    }
}

pub(crate) fn write_dependency(
    writer: &mut impl Write,
    dependency: &DependencyMetadata,
) -> ContentResult<()> {
    let path = dependency.path.to_string_lossy();

    let path_bytes = path.as_bytes();

    let path_length = u32::try_from(path_bytes.len())
        .map_err(|_| ContentError::InvalidCache("dependency path is too long".into()))?;

    write_u32(writer, path_length)?;

    writer.write_all(path_bytes)?;

    write_u64(writer, dependency.size)?;

    write_u64(writer, dependency.modified_nanos)?;

    Ok(())
}

pub(crate) fn read_dependency(reader: &mut impl Read) -> ContentResult<DependencyMetadata> {
    const MAX_PATH_LENGTH: u32 = 32 * 1024;

    let path_length = read_u32(reader)?;

    if path_length > MAX_PATH_LENGTH {
        return Err(ContentError::InvalidCache(
            "compiled dependency path is unreasonably long".into(),
        ));
    }

    let mut path_bytes = vec![0u8; path_length as usize];

    reader.read_exact(&mut path_bytes)?;

    let path_string = String::from_utf8(path_bytes).map_err(|_| {
        ContentError::InvalidCache("compiled dependency path is not valid UTF-8".into())
    })?;

    let size = read_u64(reader)?;

    let modified_nanos = read_u64(reader)?;

    Ok(DependencyMetadata {
        path: PathBuf::from(path_string),
        size,
        modified_nanos,
    })
}

/// Records the primary source first, followed by each distinct external glTF
/// buffer/image. Embedded BIN chunks and data URIs are covered by the source.
pub(crate) fn source_dependencies(path: &Path) -> ContentResult<Vec<DependencyMetadata>> {
    let primary = DependencyMetadata::from_path(path)?;
    let mut seen = HashSet::from([primary.path.clone()]);
    let mut dependencies = vec![primary];

    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or("");
    if !extension.eq_ignore_ascii_case("gltf") && !extension.eq_ignore_ascii_case("glb") {
        return Ok(dependencies);
    }

    let gltf = gltf::Gltf::open(path).map_err(|error| ContentError::ImportFailed {
        path: path.to_path_buf(),
        message: error.to_string(),
    })?;
    let base = path.parent().unwrap_or_else(|| Path::new("."));
    let uris = gltf
        .document
        .buffers()
        .filter_map(|buffer| match buffer.source() {
            gltf::buffer::Source::Uri(uri) => Some(uri),
            gltf::buffer::Source::Bin => None,
        })
        .chain(gltf.document.images().filter_map(|image| match image.source() {
            gltf::image::Source::Uri { uri, .. } => Some(uri),
            gltf::image::Source::View { .. } => None,
        }));

    // gltf::import reads all declared buffers/images, even when selecting one
    // subobject. Mirror its URI rules so cache dependencies match those reads.
    for uri in uris {
        let Some(dependency_path) = external_uri_path(base, uri)? else {
            continue;
        };
        let dependency = DependencyMetadata::from_path(&dependency_path)?;
        if seen.insert(dependency.path.clone()) {
            dependencies.push(dependency);
        }
    }

    Ok(dependencies)
}

fn external_uri_path(base: &Path, uri: &str) -> ContentResult<Option<PathBuf>> {
    if uri.starts_with("data:") {
        return Ok(None);
    }
    if let Some(path) = uri
        .strip_prefix("file://")
        .or_else(|| uri.strip_prefix("file:"))
    {
        return Ok(Some(PathBuf::from(path)));
    }
    if uri.contains(':') {
        return Err(ContentError::InvalidCache(
            "unsupported glTF dependency URI scheme".into(),
        ));
    }
    let decoded = urlencoding::decode(uri).map_err(|error| {
        ContentError::InvalidCache(format!("invalid glTF dependency URI: {error}"))
    })?;
    Ok(Some(base.join(decoded.as_ref())))
}
