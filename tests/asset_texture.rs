use rust_engine::core::config::ConfigContainer;
use rust_engine::core::{TextureFormat, TextureHandle, TextureManager};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

struct TextureTestEnv {
    root: PathBuf,
    config_path: PathBuf,
}

impl TextureTestEnv {
    fn new(name: &str) -> Self {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "rust_engine_asset_texture_{name}_{}_{}",
            std::process::id(),
            unique
        ));
        let texture_root = root.join("textures");
        std::fs::create_dir_all(texture_root.join("assets")).unwrap();

        let config_path = root.join("config.toml");
        let texture_dir = format!("{}/", texture_root.display());
        std::fs::write(
            &config_path,
            format!("[paths]\ntexture_dir = \"{}\"\n", texture_dir),
        )
        .unwrap();

        Self { root, config_path }
    }

    fn config(&self) -> ConfigContainer {
        ConfigContainer::load_from_file(&self.config_path).unwrap()
    }

    fn create_texture(&self, relative_path: &str, width: u32, height: u32) {
        let path = self.root.join("textures").join(relative_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        let image = image::RgbImage::from_fn(width, height, |x, y| {
            image::Rgb([(x % 255) as u8, (y % 255) as u8, 128])
        });
        image.save(path).unwrap();
    }
}

impl Drop for TextureTestEnv {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

#[test]
fn texture_manager_load_and_get() {
    let env = TextureTestEnv::new("load_and_get");
    env.create_texture("assets/test.png", 1024, 1024);

    let config = env.config();
    let mut mgr = TextureManager::new(config.get_config());

    // テクスチャをロード
    let handle = mgr.load("assets/test.png").unwrap();
    assert!(handle.is_valid());
    assert_eq!(mgr.loaded_count(), 1);

    // データを取得
    let data = mgr.get(&handle).expect("テクスチャが取得できない");
    assert_eq!(data.width, 1024);
    assert_eq!(data.height, 1024);
    assert_eq!(data.format, TextureFormat::Rgba8);
    assert_eq!(data.data.len(), 1024 * 1024 * 3);
}

#[test]
fn texture_manager_duplicate_load_returns_same_handle() {
    let env = TextureTestEnv::new("duplicate_load");
    env.create_texture("assets/tex1.png", 16, 16);

    let config = env.config();
    let mut mgr = TextureManager::new(config.get_config());

    let h1 = mgr.load("assets/tex1.png").unwrap();
    let h2 = mgr.load("assets/tex1.png").unwrap();

    // 同じパスをロードしたら同じハンドルが返る（キャッシュ）
    assert_eq!(h1, h2);
    assert_eq!(mgr.loaded_count(), 1);
}

#[test]
fn texture_manager_unload() {
    let env = TextureTestEnv::new("unload");
    env.create_texture("assets/tex1.png", 16, 16);

    let config = env.config();
    let mut mgr = TextureManager::new(config.get_config());

    let handle = mgr.load("assets/tex1.png").unwrap();
    assert_eq!(mgr.loaded_count(), 1);

    // アンロード
    mgr.unload(handle);
    assert_eq!(mgr.loaded_count(), 0);
    assert!(mgr.get(&handle).is_none());
}

#[test]
fn texture_handle_invalid() {
    let invalid = TextureHandle::invalid();
    assert!(!invalid.is_valid());
    assert_eq!(invalid.id(), 0);

    let env = TextureTestEnv::new("invalid_handle");
    let config = env.config();
    let mgr = TextureManager::new(config.get_config());
    assert!(mgr.get(&invalid).is_none());
}

#[test]
fn texture_manager_multiple_textures() {
    let env = TextureTestEnv::new("multiple_textures");
    env.create_texture("assets/tex1.png", 16, 16);
    env.create_texture("assets/tex2.png", 32, 16);
    env.create_texture("assets/tex3.png", 16, 32);

    let config = env.config();
    let mut mgr = TextureManager::new(config.get_config());

    let h1 = mgr.load("assets/tex1.png").unwrap();
    let h2 = mgr.load("assets/tex2.png").unwrap();
    let h3 = mgr.load("assets/tex3.png").unwrap();

    assert_ne!(h1, h2);
    assert_ne!(h2, h3);
    assert_ne!(h1, h3);
    assert_eq!(mgr.loaded_count(), 3);

    // それぞれ取得可能
    assert!(mgr.get(&h1).is_some());
    assert!(mgr.get(&h2).is_some());
    assert!(mgr.get(&h3).is_some());
}
