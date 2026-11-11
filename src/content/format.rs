use std::io::{
    Read,
    Write,
};

use super::{
    ContentError,
    ContentResult,
};

pub(crate) const MAGIC: &[u8; 8] =
    b"OXASSET\0";

pub(crate) const FORMAT_VERSION: u32 = 3;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
#[repr(u32)]
pub enum AssetType {
    Texture = 1,
    Mesh = 2,
    Material = 3,
    Audio = 4,
}

impl AssetType {
    fn from_u32(
        value: u32,
    ) -> Option<Self> {
        match value {
            1 => Some(Self::Texture),
            2 => Some(Self::Mesh),
            3 => Some(Self::Material),
            4 => Some(Self::Audio),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub(crate) struct AssetHeader {
    pub asset_type: AssetType,
    pub compiler_version: u32,
    pub dependency_count: u32,
    pub subobject_length: u32,
    pub payload_size: u64,
}

pub(crate) fn write_header(
    writer: &mut impl Write,
    header: &AssetHeader,
) -> ContentResult<()> {
    writer.write_all(MAGIC)?;

    writer.write_all(
        &FORMAT_VERSION.to_le_bytes()
    )?;

    writer.write_all(
        &(header.asset_type as u32)
            .to_le_bytes()
    )?;

    writer.write_all(
        &header
            .compiler_version
            .to_le_bytes()
    )?;

    writer.write_all(
        &header
            .dependency_count
            .to_le_bytes()
    )?;

    writer.write_all(
        &header
            .subobject_length
            .to_le_bytes()
    )?;

    writer.write_all(
        &header
            .payload_size
            .to_le_bytes()
    )?;

    Ok(())
}

pub(crate) fn read_header(
    reader: &mut impl Read,
) -> ContentResult<AssetHeader> {
    let mut magic = [0u8; 8];

    reader.read_exact(
        &mut magic
    )?;

    if &magic != MAGIC {
        return Err(
            ContentError::InvalidCache(
                "invalid Oxide asset magic"
                    .into()
            )
        );
    }

    let version =
        read_u32(reader)?;

    if version != FORMAT_VERSION {
        return Err(
            ContentError::InvalidCache(
                format!(
                    "unsupported Oxide asset version {version}"
                )
            )
        );
    }

    let asset_type_raw =
        read_u32(reader)?;

    let Some(asset_type) =
        AssetType::from_u32(
            asset_type_raw
        )
    else {
        return Err(
            ContentError::InvalidCache(
                format!(
                    "unknown Oxide asset type {asset_type_raw}"
                )
            )
        );
    };

    let compiler_version =
        read_u32(reader)?;

    let dependency_count =
        read_u32(reader)?;

    let subobject_length =
        read_u32(reader)?;

    let payload_size =
        read_u64(reader)?;

    Ok(
        AssetHeader {
            asset_type,
            compiler_version,
            dependency_count,
            subobject_length,
            payload_size,
        }
    )
}

pub(crate) fn write_u32(
    writer: &mut impl Write,
    value: u32,
) -> ContentResult<()> {
    writer.write_all(
        &value.to_le_bytes()
    )?;

    Ok(())
}

pub(crate) fn write_u64(
    writer: &mut impl Write,
    value: u64,
) -> ContentResult<()> {
    writer.write_all(
        &value.to_le_bytes()
    )?;

    Ok(())
}

pub(crate) fn read_u32(
    reader: &mut impl Read,
) -> ContentResult<u32> {
    let mut bytes = [0u8; 4];

    reader.read_exact(
        &mut bytes
    )?;

    Ok(
        u32::from_le_bytes(bytes)
    )
}

pub(crate) fn read_u64(
    reader: &mut impl Read,
) -> ContentResult<u64> {
    let mut bytes = [0u8; 8];

    reader.read_exact(
        &mut bytes
    )?;

    Ok(
        u64::from_le_bytes(bytes)
    )
}