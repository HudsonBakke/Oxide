use std::{
    io::{
        Read,
        Write,
    },
    path::Path,
};

use super::{
    format::AssetType,
    ContentResult,
};

/// A fundamental Oxide asset that can be loaded by ContentManager.
///
/// This trait is sealed. Oxide defines which engine primitives
/// are valid content assets.
pub trait ContentAsset:
    private::ContentAssetInternal
{
}

pub(crate) use private::ContentAssetInternal;

mod private {
    use super::*;

    pub trait ContentAssetInternal:
        Sized
    {
        fn asset_type_name()
            -> &'static str;

        fn supports_extension(
            extension: &str,
        ) -> bool;

        fn import(
            path: &Path,
            subobject: &str,
        ) -> ContentResult<Self>;

        fn compiled_asset_type()
            -> AssetType;

        fn compiler_version()
            -> u32;

        fn write_compiled(
            &self,
            writer: &mut impl Write,
        ) -> ContentResult<()>;

        fn read_compiled(
            reader: &mut impl Read,
            payload_size: u64,
        ) -> ContentResult<Self>;
    }
}