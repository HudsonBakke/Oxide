use std::{
    io::{Read, Write},
    path::Path,
};

use super::{ContentResult, format::AssetType};

/// A fundamental Oxide asset that can be loaded by ContentManager.
///
/// This trait is sealed. Oxide defines which engine primitives
/// are valid content assets.
pub trait ContentAsset: private::ContentAssetInternal {}

pub(crate) use private::ContentAssetInternal;

mod private {
    use super::*;

    /// Internal import and serialization contract behind the sealed ContentAsset trait.
    pub trait ContentAssetInternal: Sized {
        /// Returns the asset type name used in content diagnostics.
        fn asset_type_name() -> &'static str;

        /// Whether this asset type can import the normalized source extension.
        fn supports_extension(extension: &str) -> bool;

        /// Imports the selected source object without consulting the compiled cache.
        fn import(path: &Path, subobject: &str) -> ContentResult<Self>;

        /// Returns the type identifier stored in compiled cache headers.
        fn compiled_asset_type() -> AssetType;

        /// Returns the serializer/importer revision required for this asset payload.
        fn compiler_version() -> u32;

        /// Serializes this asset payload into the cache stream.
        fn write_compiled(&self, writer: &mut impl Write) -> ContentResult<()>;

        /// Deserializes and validates an asset payload with the declared byte length.
        fn read_compiled(reader: &mut impl Read, payload_size: u64) -> ContentResult<Self>;
    }
}
