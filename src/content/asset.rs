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

pub trait ContentAsset: Sized {
    fn asset_type_name()
        -> &'static str;

    fn supports_extension(
        extension: &str,
    ) -> bool;

    fn import(
        path: &Path,
        subobject: &str,
    ) -> ContentResult<Self>;

    #[doc(hidden)]
    fn compiled_asset_type()
        -> AssetType;

    #[doc(hidden)]
    fn write_compiled(
        &self,
        writer: &mut impl Write,
    ) -> ContentResult<()>;

    #[doc(hidden)]
    fn read_compiled(
        reader: &mut impl Read,
        payload_size: u64,
    ) -> ContentResult<Self>;
}