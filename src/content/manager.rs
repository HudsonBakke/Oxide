use std::{
    fs::{
        self,
        File,
    },
    io::{
        BufReader,
        BufWriter,
        Read,
        Seek,
        SeekFrom,
        Write,
    },
    path::{
        Path,
        PathBuf,
    },
};

use super::{
    cache,
    format::{
        self,
        AssetHeader,
    },
    metadata::{
        self,
        DependencyMetadata,
    },
    ContentAsset,
    ContentError,
    ContentResult,
};

#[derive(Debug)]
pub struct ContentManager {
    cache_directory: PathBuf,
}

impl ContentManager {
    /// Creates a content manager using Oxide's default
    /// compiled-content cache directory.
    pub fn new() -> Self {
        Self {
            cache_directory:
                PathBuf::from(
                    "target/oxide-content"
                ),
        }
    }

    /// Creates a content manager with a custom compiled-content
    /// cache directory.
    pub fn with_cache_directory(
        path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            cache_directory:
                path.into(),
        }
    }

    /// Loads the default asset from a source file.
    pub fn load<T: ContentAsset>(
        &mut self,
        path: impl AsRef<Path>,
    ) -> ContentResult<T> {
        self.load_internal::<T>(
            path.as_ref(),
            "",
        )
    }

    /// Loads a named subobject from a source file.
    pub fn load_sub<T: ContentAsset>(
        &mut self,
        path: impl AsRef<Path>,
        subobject: impl AsRef<str>,
    ) -> ContentResult<T> {
        let subobject =
            subobject.as_ref();

        if subobject.is_empty() {
            return Err(
                ContentError::InvalidSubObject {
                    message:
                        "subobject name cannot be empty; use load for the default object",
                }
            );
        }

        self.load_internal::<T>(
            path.as_ref(),
            subobject,
        )
    }

    fn load_internal<T: ContentAsset>(
        &mut self,
        requested_path: &Path,
        subobject: &str,
    ) -> ContentResult<T> {
        if !requested_path.exists() {
            return Err(
                ContentError::FileNotFound {
                    path:
                        requested_path
                            .to_path_buf(),
                }
            );
        }

        let canonical_path =
            fs::canonicalize(
                requested_path
            )?;

        self.validate_source_type::<T>(
            &canonical_path
        )?;

        let cache_path =
            cache::cache_path(
                &self.cache_directory,
                &canonical_path,
                T::compiled_asset_type(),
                subobject,
            );

        // Cache failures are deliberately ignored.
        // The source asset is authoritative.
        if cache_path.is_file() {
            if let Ok(asset) =
                self.try_load_compiled::<T>(
                    &canonical_path,
                    &cache_path,
                    subobject,
                )
            {
                return Ok(asset);
            }
        }

        let asset =
            T::import(
                &canonical_path,
                subobject,
            )?;

        // Failure to write the cache should not prevent
        // an otherwise valid source asset from loading.
        let _ =
            self.write_compiled::<T>(
                &canonical_path,
                &cache_path,
                subobject,
                &asset,
            );

        Ok(asset)
    }

    pub fn cache_directory(
        &self,
    ) -> &Path {
        &self.cache_directory
    }

    fn validate_source_type<T: ContentAsset>(
        &self,
        path: &Path,
    ) -> ContentResult<()> {
        let extension =
            path.extension()
                .and_then(
                    |value| {
                        value.to_str()
                    }
                )
                .map(
                    |value| {
                        value.to_ascii_lowercase()
                    }
                );

        let Some(extension) =
            extension
        else {
            return Err(
                ContentError::UnsupportedFormat {
                    path:
                        path.to_path_buf(),
                    extension: None,
                }
            );
        };

        if !T::supports_extension(
            &extension
        ) {
            return Err(
                ContentError::UnsupportedAssetType {
                    path:
                        path.to_path_buf(),

                    asset_type:
                        T::asset_type_name(),

                    extension:
                        Some(extension),
                }
            );
        }

        Ok(())
    }

    fn try_load_compiled<T: ContentAsset>(
        &self,
        source_path: &Path,
        cache_path: &Path,
        subobject: &str,
    ) -> ContentResult<T> {
        let file =
            File::open(
                cache_path
            )?;

        let file_size =
            file.metadata()?.len();

        let mut reader =
            BufReader::new(file);

        let header =
            format::read_header(
                &mut reader
            )?;

        if header.asset_type
            != T::compiled_asset_type()
        {
            return Err(
                ContentError::InvalidCache(
                    "compiled asset type does not match requested asset type"
                        .into()
                )
            );
        }

        if header.compiler_version
            != T::compiler_version()
        {
            return Err(
                ContentError::InvalidCache(
                    format!(
                        "compiled asset compiler version {} does not match current compiler version {}",
                        header.compiler_version,
                        T::compiler_version(),
                    )
                )
            );
        }

        // Protect against absurd dependency counts in a
        // corrupt or malicious cache file before allocating or
        // looping over them.
        const MAX_DEPENDENCIES: u32 =
            16_384;

        if header.dependency_count
            > MAX_DEPENDENCIES
        {
            return Err(
                ContentError::InvalidCache(
                    "compiled asset contains too many dependencies"
                        .into()
                )
            );
        }

        let mut dependencies =
            Vec::with_capacity(
                header.dependency_count
                    as usize
            );

        for _ in 0
            ..header.dependency_count
        {
            dependencies.push(
                metadata::read_dependency(
                    &mut reader
                )?
            );
        }

        // Every compiled asset currently has at least its
        // primary source as a dependency.
        if dependencies.is_empty() {
            return Err(
                ContentError::InvalidCache(
                    "compiled asset has no source dependency"
                        .into()
                )
            );
        }

        // The first dependency is always the primary source.
        if dependencies[0].path
            != source_path
        {
            return Err(
                ContentError::InvalidCache(
                    "compiled asset source does not match requested source"
                        .into()
                )
            );
        }

        if dependencies
            .iter()
            .any(
                |dependency| {
                    !dependency.is_current()
                }
            )
        {
            return Err(
                ContentError::InvalidCache(
                    "compiled asset dependencies are stale"
                        .into()
                )
            );
        }

        const MAX_SUBOBJECT_LENGTH: u32 =
            64 * 1024;

        if header.subobject_length
            > MAX_SUBOBJECT_LENGTH
        {
            return Err(
                ContentError::InvalidCache(
                    "compiled asset subobject name is unreasonably long"
                        .into()
                )
            );
        }

        let mut subobject_bytes =
            vec![
                0u8;
                header.subobject_length
                    as usize
            ];

        reader.read_exact(
            &mut subobject_bytes
        )?;

        let cached_subobject =
            String::from_utf8(
                subobject_bytes
            )
            .map_err(
                |_| {
                    ContentError::InvalidCache(
                        "compiled asset subobject is not valid UTF-8"
                            .into()
                    )
                }
            )?;

        if cached_subobject
            != subobject
        {
            return Err(
                ContentError::InvalidCache(
                    "compiled asset subobject does not match requested subobject"
                        .into()
                )
            );
        }

        // Verify the declared payload fits exactly within the
        // file. This catches truncated files and garbage
        // appended to otherwise valid cache data.
        let payload_start =
            reader.stream_position()?;

        let expected_file_size =
            payload_start
                .checked_add(
                    header.payload_size
                )
                .ok_or_else(
                    || {
                        ContentError::InvalidCache(
                            "compiled asset size overflow"
                                .into()
                        )
                    }
                )?;

        if expected_file_size
            != file_size
        {
            return Err(
                ContentError::InvalidCache(
                    "compiled asset payload size does not match file size"
                        .into()
                )
            );
        }

        T::read_compiled(
            &mut reader,
            header.payload_size,
        )
    }

    fn write_compiled<T: ContentAsset>(
        &self,
        source_path: &Path,
        cache_path: &Path,
        subobject: &str,
        asset: &T,
    ) -> ContentResult<()> {
        fs::create_dir_all(
            &self.cache_directory
        )?;

        let dependency =
            DependencyMetadata::from_path(
                source_path
            )?;

        let temporary_path =
            temporary_cache_path(
                cache_path
            );

        // Remove a stale temporary file from a previous
        // interrupted compilation if one exists.
        match fs::remove_file(
            &temporary_path
        ) {
            Ok(()) => {}

            Err(error)
                if error.kind()
                    == std::io::ErrorKind::NotFound =>
            {
            }

            Err(error) => {
                return Err(
                    ContentError::Io(
                        error
                    )
                );
            }
        }

        let file =
            File::create(
                &temporary_path
            )?;

        let mut writer =
            BufWriter::new(file);

            
        let subobject_length =
            u32::try_from(
                subobject.as_bytes().len()
            )
            .map_err(
                |_| {
                    ContentError::InvalidCache(
                        "content subobject name is too long"
                            .into()
                    )
                }
            )?;

        // We don't know the payload size until the asset has
        // serialized itself. Write a placeholder header first.
        let placeholder_header =
            AssetHeader {
                asset_type:
                    T::compiled_asset_type(),

                compiler_version:
                    T::compiler_version(),

                dependency_count: 1,

                subobject_length,

                payload_size: 0,
            };

        format::write_header(
            &mut writer,
            &placeholder_header,
        )?;

        metadata::write_dependency(
            &mut writer,
            &dependency,
        )?;

        let payload_start =
            writer.stream_position()?;

        asset.write_compiled(
            &mut writer
        )?;

        let payload_end =
            writer.stream_position()?;

        let payload_size =
            payload_end
                .checked_sub(
                    payload_start
                )
                .ok_or_else(
                    || {
                        ContentError::InvalidCache(
                            "compiled payload position is invalid"
                                .into()
                        )
                    }
                )?;

        // Rewrite the header now that we know the actual
        // payload size.
        writer.seek(
            SeekFrom::Start(0)
        )?;

        let final_header =
            AssetHeader {
                asset_type:
                    T::compiled_asset_type(),

                compiler_version:
                    T::compiler_version(),

                dependency_count: 1,

                subobject_length,

                payload_size,
            };

        format::write_header(
            &mut writer,
            &final_header,
        )?;

        writer.flush()?;

        // Ensure BufWriter releases the file handle before
        // attempting the rename on Windows.
        drop(writer);

        replace_cache_file(
            &temporary_path,
            cache_path,
        )?;

        Ok(())
    }
}

impl Default for ContentManager {
    fn default() -> Self {
        Self::new()
    }
}

fn temporary_cache_path(
    cache_path: &Path,
) -> PathBuf {
    let mut path =
        cache_path
            .as_os_str()
            .to_os_string();

    path.push(".tmp");

    PathBuf::from(path)
}

fn replace_cache_file(
    temporary_path: &Path,
    cache_path: &Path,
) -> ContentResult<()> {
    // On Windows, rename() does not reliably replace an
    // existing destination file. Remove the old disposable
    // cache first.
    match fs::remove_file(
        cache_path
    ) {
        Ok(()) => {}

        Err(error)
            if error.kind()
                == std::io::ErrorKind::NotFound =>
        {
        }

        Err(error) => {
            return Err(
                ContentError::Io(
                    error
                )
            );
        }
    }

    fs::rename(
        temporary_path,
        cache_path,
    )?;

    Ok(())
}