use std::{
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use image::{ImageBuffer, Rgba};
use oxide::{
    content::ContentManager,
    graphics::{Material, Mesh, Texture},
};

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

struct Fixture {
    root: PathBuf,
    source: PathBuf,
    buffer: PathBuf,
    image: PathBuf,
    cache: PathBuf,
}

impl Fixture {
    fn new(binary: bool) -> Self {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let sequence = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
        let root = PathBuf::from(format!(
            "target/test-content-cache/dependencies-{}-{unique}-{sequence}",
            std::process::id(),
        ));
        fs::create_dir_all(&root).unwrap();
        let source = root.join(if binary { "scene.glb" } else { "scene.gltf" });
        let fixture = Self {
            source,
            buffer: root.join("mesh data.bin"),
            image: root.join("surface image.png"),
            cache: root.join("cache"),
            root,
        };
        fixture.write_buffer(1.0, false);
        fixture.write_image(2, 2);

        // URI escapes deliberately differ from the filenames on disk.
        let json = r#"{
            "asset": {"version": "2.0"},
            "buffers": [{"uri": "mesh%20data.bin", "byteLength": 36}],
            "bufferViews": [{"buffer": 0, "byteLength": 36}],
            "accessors": [{
                "bufferView": 0, "componentType": 5126, "count": 3,
                "type": "VEC3", "min": [0, 0, 0], "max": [2, 2, 0]
            }],
            "meshes": [{"name": "Triangle", "primitives": [{
                "attributes": {"POSITION": 0}
            }]}],
            "nodes": [{"name": "TriangleNode", "mesh": 0}],
            "scenes": [{"nodes": [0]}],
            "scene": 0,
            "images": [{"uri": "surface%20image.png"}],
            "textures": [{"name": "Surface", "source": 0}],
            "materials": [{"name": "Paint", "pbrMetallicRoughness": {
                "baseColorTexture": {"index": 0}
            }}]
        }"#;
        if binary {
            // A GLB may reference external files without containing a BIN chunk.
            let mut json = json.as_bytes().to_vec();
            while json.len() % 4 != 0 {
                json.push(b' ');
            }
            let mut glb = Vec::new();
            glb.extend_from_slice(b"glTF");
            glb.extend_from_slice(&2_u32.to_le_bytes());
            glb.extend_from_slice(&(20 + json.len() as u32).to_le_bytes());
            glb.extend_from_slice(&(json.len() as u32).to_le_bytes());
            glb.extend_from_slice(b"JSON");
            glb.extend_from_slice(&json);
            fs::write(&fixture.source, glb).unwrap();
        } else {
            fs::write(&fixture.source, json).unwrap();
        }
        fixture
    }

    fn content(&self) -> ContentManager {
        ContentManager::with_cache_directory(&self.cache)
    }

    fn write_buffer(&self, scale: f32, padding: bool) {
        let mut bytes = Vec::new();
        for value in [0.0_f32, 0.0, 0.0, scale, 0.0, 0.0, 0.0, scale, 0.0] {
            bytes.extend_from_slice(&value.to_le_bytes());
        }
        if padding {
            // Also change the size: this does not depend on timestamp precision.
            bytes.extend_from_slice(&[0; 4]);
        }
        fs::write(&self.buffer, bytes).unwrap();
    }

    fn write_image(&self, width: u32, height: u32) {
        ImageBuffer::from_pixel(width, height, Rgba([255_u8, 0, 0, 255]))
            .save(&self.image)
            .unwrap();
    }

    fn cache_file(&self) -> PathBuf {
        let paths: Vec<_> = fs::read_dir(&self.cache)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .collect();
        assert_eq!(paths.len(), 1);
        paths[0].clone()
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn mark_cache(path: &Path) -> SystemTime {
    // An old, representable time makes a rewrite observable without sleeps,
    // including on filesystems with coarse modification-time resolution.
    fs::OpenOptions::new()
        .write(true)
        .open(path)
        .unwrap()
        .set_modified(UNIX_EPOCH + Duration::from_secs(1_000_000_000))
        .unwrap();
    fs::metadata(path).unwrap().modified().unwrap()
}

#[test]
fn named_mesh_material_and_texture_caches_are_reused() {
    for binary in [false, true] {
        // Separate caches let us prove reuse for each public asset type.
        let fixture = Fixture::new(binary);
        let first = fixture
            .content()
            .load_sub::<Mesh>(&fixture.source, "TriangleNode")
            .unwrap();
        let cache = fixture.cache_file();
        let marked = mark_cache(&cache);
        let second = fixture
            .content()
            .load_sub::<Mesh>(&fixture.source, "TriangleNode")
            .unwrap();
        assert_eq!(first.vertices().len(), second.vertices().len());
        assert_eq!(fs::metadata(cache).unwrap().modified().unwrap(), marked);

        let fixture = Fixture::new(binary);
        fixture
            .content()
            .load_sub::<Material>(&fixture.source, "Paint")
            .unwrap();
        let cache = fixture.cache_file();
        let marked = mark_cache(&cache);
        let material = fixture
            .content()
            .load_sub::<Material>(&fixture.source, "Paint")
            .unwrap();
        assert_eq!(material.base_texture().width(), 2);
        assert_eq!(fs::metadata(cache).unwrap().modified().unwrap(), marked);

        let fixture = Fixture::new(binary);
        fixture
            .content()
            .load_sub::<Texture>(&fixture.source, "Surface")
            .unwrap();
        let cache = fixture.cache_file();
        let marked = mark_cache(&cache);
        let texture = fixture
            .content()
            .load_sub::<Texture>(&fixture.source, "Surface")
            .unwrap();
        assert_eq!((texture.width(), texture.height()), (2, 2));
        assert_eq!(fs::metadata(cache).unwrap().modified().unwrap(), marked);
    }
}

#[test]
fn changed_external_buffer_rebuilds_mesh_for_gltf_and_glb() {
    for binary in [false, true] {
        let fixture = Fixture::new(binary);
        let source_bytes = fs::read(&fixture.source).unwrap();
        let first = fixture.content().load::<Mesh>(&fixture.source).unwrap();
        assert_eq!(first.vertices()[1].position.x, 1.0);
        fixture.write_buffer(2.0, true);
        let second = fixture.content().load::<Mesh>(&fixture.source).unwrap();
        assert_eq!(second.vertices()[1].position.x, 2.0);
        assert_eq!(fs::read(&fixture.source).unwrap(), source_bytes);
    }
}

#[test]
fn changed_external_image_rebuilds_texture_and_material() {
    for binary in [false, true] {
        let fixture = Fixture::new(binary);
        let source_bytes = fs::read(&fixture.source).unwrap();
        let texture = fixture.content().load::<Texture>(&fixture.source).unwrap();
        let material = fixture.content().load::<Material>(&fixture.source).unwrap();
        assert_eq!(texture.width(), 2);
        assert_eq!(material.base_texture().width(), 2);
        fixture.write_image(31, 17);
        let texture = fixture.content().load::<Texture>(&fixture.source).unwrap();
        let material = fixture.content().load::<Material>(&fixture.source).unwrap();
        assert_eq!((texture.width(), texture.height()), (31, 17));
        assert_eq!(material.base_texture().width(), 31);
        assert_eq!(fs::read(&fixture.source).unwrap(), source_bytes);
    }
}

#[test]
fn missing_external_files_do_not_return_stale_assets() {
    let fixture = Fixture::new(false);
    fixture.content().load::<Mesh>(&fixture.source).unwrap();
    fs::remove_file(&fixture.buffer).unwrap();
    assert!(fixture.content().load::<Mesh>(&fixture.source).is_err());

    let fixture = Fixture::new(false);
    fixture.content().load::<Texture>(&fixture.source).unwrap();
    fixture.content().load::<Material>(&fixture.source).unwrap();
    fs::remove_file(&fixture.image).unwrap();
    assert!(fixture.content().load::<Texture>(&fixture.source).is_err());
    assert!(fixture.content().load::<Material>(&fixture.source).is_err());
}

#[test]
fn previous_cache_version_is_rebuilt_automatically() {
    let fixture = Fixture::new(false);
    fixture.content().load::<Texture>(&fixture.source).unwrap();
    let cache = fixture.cache_file();
    let mut bytes = fs::read(&cache).unwrap();
    bytes[8..12].copy_from_slice(&3_u32.to_le_bytes());
    fs::write(&cache, bytes).unwrap();
    let texture = fixture.content().load::<Texture>(&fixture.source).unwrap();
    assert_eq!(texture.width(), 2);
    let bytes = fs::read(cache).unwrap();
    assert_eq!(u32::from_le_bytes(bytes[8..12].try_into().unwrap()), 4);
}

#[test]
fn same_size_buffer_edit_is_detected_by_modification_time() {
    let fixture = Fixture::new(false);
    fixture.content().load::<Mesh>(&fixture.source).unwrap();
    let original_size = fs::metadata(&fixture.buffer).unwrap().len();
    fixture.write_buffer(2.0, false);
    mark_cache(&fixture.buffer);
    assert_eq!(fs::metadata(&fixture.buffer).unwrap().len(), original_size);
    let mesh = fixture.content().load::<Mesh>(&fixture.source).unwrap();
    assert_eq!(mesh.vertices()[1].position.x, 2.0);
}

#[test]
fn embedded_data_uri_buffer_does_not_require_an_external_file() {
    let fixture = Fixture::new(false);
    let json = fs::read_to_string(&fixture.source).unwrap().replace(
        "mesh%20data.bin",
        "data:application/octet-stream;base64,AAAAAAAAAAAAAAAAAACAPwAAAAAAAAAAAAAAAAAAgD8AAAAA",
    );
    fs::write(&fixture.source, json).unwrap();
    fs::remove_file(&fixture.buffer).unwrap();
    fixture.content().load::<Mesh>(&fixture.source).unwrap();
    let cache = fixture.cache_file();
    let marked = mark_cache(&cache);
    let mesh = fixture.content().load::<Mesh>(&fixture.source).unwrap();
    assert_eq!(mesh.vertices()[1].position.x, 1.0);
    assert_eq!(fs::metadata(cache).unwrap().modified().unwrap(), marked);
}
