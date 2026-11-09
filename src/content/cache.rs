use std::path::{
    Path,
    PathBuf,
};

use super::format::AssetType;

const FNV_OFFSET_BASIS:
    u64 = 0xcbf29ce484222325;

const FNV_PRIME:
    u64 = 0x100000001b3;

fn hash_bytes(
    mut hash: u64,
    bytes: &[u8],
) -> u64 {
    for byte in bytes {
        hash ^= *byte as u64;

        hash =
            hash.wrapping_mul(
                FNV_PRIME
            );
    }

    hash
}

pub(crate) fn cache_key(
    source_path: &Path,
    asset_type: AssetType,
    subobject: &str,
) -> u64 {
    let mut hash =
        FNV_OFFSET_BASIS;

    hash = hash_bytes(
        hash,
        source_path
            .to_string_lossy()
            .as_bytes(),
    );

    // Separator prevents ambiguous concatenations.
    hash = hash_bytes(
        hash,
        &[0],
    );

    hash = hash_bytes(
        hash,
        &(asset_type as u32)
            .to_le_bytes(),
    );

    hash = hash_bytes(
        hash,
        &[0],
    );

    hash_bytes(
        hash,
        subobject.as_bytes(),
    )
}

pub(crate) fn cache_path(
    cache_directory: &Path,
    source_path: &Path,
    asset_type: AssetType,
    subobject: &str,
) -> PathBuf {
    let key =
        cache_key(
            source_path,
            asset_type,
            subobject,
        );

    cache_directory.join(
        format!(
            "{key:016x}.oxasset"
        )
    )
}