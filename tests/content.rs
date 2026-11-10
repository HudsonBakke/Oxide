use oxide::{
    content::{
        ContentError,
        ContentManager,
    },
    graphics::{
        Texture,
        Mesh,
    },
};

use std::{
    fs,
    path::PathBuf,
    time::{
        SystemTime,
        UNIX_EPOCH,
    },
};

use image::{
    ImageBuffer,
    Rgba,
};

fn test_cache_directory(
    name: &str,
) -> PathBuf {
    let unique =
        SystemTime::now()
            .duration_since(
                UNIX_EPOCH
            )
            .unwrap()
            .as_nanos();

    PathBuf::from(
        format!(
            "target/test-content-cache/{name}-{unique}"
        )
    )
}

#[test]
fn loads_supported_texture() {
    let mut content =
        ContentManager::new();

    let texture =
        content
            .load::<Texture>(
                "assets/brick.jpg"
            )
            .expect(
                "texture should load"
            );

    assert!(
        texture.width() > 0
    );

    assert!(
        texture.height() > 0
    );
}

#[test]
fn missing_file_reports_file_not_found() {
    let mut content =
        ContentManager::new();

    let result =
        content.load::<Texture>(
            "assets/definitely_missing.png"
        );

    assert!(
        matches!(
            result,
            Err(
                ContentError::FileNotFound {
                    ..
                }
            )
        )
    );
}

#[test]
fn loading_texture_creates_compiled_cache() {
    let cache =
        test_cache_directory(
            "creates-cache"
        );

    let mut content =
        ContentManager::with_cache_directory(
            &cache
        );

    content
        .load::<Texture>(
            "assets/brick.jpg"
        )
        .expect(
            "texture should load"
        );

    let files: Vec<_> =
        fs::read_dir(
            &cache
        )
        .expect(
            "cache directory should exist"
        )
        .collect();

    assert_eq!(
        files.len(),
        1,
        "exactly one compiled asset should be created"
    );

    fs::remove_dir_all(
        cache
    ).unwrap();
}

#[test]
fn current_cache_is_not_rewritten() {
    let cache =
        test_cache_directory(
            "reuse-cache"
        );

    let mut content =
        ContentManager::with_cache_directory(
            &cache
        );

    content
        .load::<Texture>(
            "assets/brick.jpg"
        )
        .unwrap();

    let cache_file =
        fs::read_dir(
            &cache
        )
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();

    let first_modified =
        fs::metadata(
            &cache_file
        )
        .unwrap()
        .modified()
        .unwrap();

    // A second load should deserialize the existing cache
    // rather than rewriting it.
    content
        .load::<Texture>(
            "assets/brick.jpg"
        )
        .unwrap();

    let second_modified =
        fs::metadata(
            &cache_file
        )
        .unwrap()
        .modified()
        .unwrap();

    assert_eq!(
        first_modified,
        second_modified
    );

    fs::remove_dir_all(
        cache
    ).unwrap();
}

#[test]
fn corrupt_cache_is_rebuilt() {
    let cache =
        test_cache_directory(
            "corrupt-cache"
        );

    let mut content =
        ContentManager::with_cache_directory(
            &cache
        );

    let original =
        content
            .load::<Texture>(
                "assets/brick.jpg"
            )
            .unwrap();

    let cache_file =
        fs::read_dir(
            &cache
        )
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();

    fs::write(
        &cache_file,
        b"this is not an Oxide asset",
    )
    .unwrap();

    let rebuilt =
        content
            .load::<Texture>(
                "assets/brick.jpg"
            )
            .expect(
                "corrupt cache should be rebuilt from source"
            );

    assert_eq!(
        rebuilt.width(),
        original.width()
    );

    assert_eq!(
        rebuilt.height(),
        original.height()
    );

    let rebuilt_size =
        fs::metadata(
            &cache_file
        )
        .unwrap()
        .len();

    assert!(
        rebuilt_size
            > b"this is not an Oxide asset".len()
                as u64
    );

    fs::remove_dir_all(
        cache
    ).unwrap();
}

fn write_test_png(
    path: &std::path::Path,
    width: u32,
    height: u32,
) {
    let image =
        ImageBuffer::from_pixel(
            width,
            height,
            Rgba([
                255u8,
                0,
                0,
                255,
            ]),
        );

    image
        .save(path)
        .unwrap();
}

#[test]
fn changed_source_rebuilds_cache() {
    let root =
        test_cache_directory(
            "stale-source"
        );

    let source_directory =
        root.join("source");

    let cache_directory =
        root.join("cache");

    fs::create_dir_all(
        &source_directory
    ).unwrap();

    let source =
        source_directory.join(
            "texture.png"
        );

    write_test_png(
        &source,
        2,
        2,
    );

    let mut content =
        ContentManager::with_cache_directory(
            &cache_directory
        );

    let first =
        content
            .load::<Texture>(
                &source
            )
            .unwrap();

    assert_eq!(
        first.width(),
        2
    );

    assert_eq!(
        first.height(),
        2
    );

    // Changing dimensions guarantees both file contents and
    // normally file size change, so stale detection does not
    // depend solely on timestamp precision.
    write_test_png(
        &source,
        4,
        3,
    );

    let second =
        content
            .load::<Texture>(
                &source
            )
            .unwrap();

    assert_eq!(
        second.width(),
        4
    );

    assert_eq!(
        second.height(),
        3
    );

    fs::remove_dir_all(
        root
    ).unwrap();
}

#[test]
fn texture_rejects_subobject() {
    let cache =
        test_cache_directory(
            "texture-subobject"
        );

    let mut content =
        ContentManager::with_cache_directory(
            &cache
        );

    let result =
        content.load_sub::<Texture>(
            "assets/brick.jpg",
            "NotARealSubobject",
        );

    assert!(
        matches!(
            result,
            Err(
                ContentError::SubObjectNotSupported {
                    ..
                }
            )
        )
    );

    if cache.exists() {
        fs::remove_dir_all(
            cache
        ).unwrap();
    }
}

#[test]
fn empty_selector_path_still_loads_normally() {
    let cache =
        test_cache_directory(
            "ordinary-load"
        );

    let mut content =
        ContentManager::with_cache_directory(
            &cache
        );

    let texture =
        content
            .load::<Texture>(
                "assets/brick.jpg"
            )
            .unwrap();

    assert!(
        texture.width() > 0
    );

    assert!(
        texture.height() > 0
    );

    fs::remove_dir_all(
        cache
    ).unwrap();
}

#[test]
fn loads_glb_mesh() {
    let cache =
        test_cache_directory(
            "glb-mesh"
        );

    let mut content =
        ContentManager::with_cache_directory(
            &cache
        );

    let mesh =
        content
            .load::<Mesh>(
                "assets/blender_cube.glb"
            )
            .expect(
                "GLB mesh should load"
            );

    assert!(
        !mesh.vertices().is_empty()
    );

    assert!(
        !mesh.indices().is_empty()
    );

    fs::remove_dir_all(
        cache
    ).unwrap();
}