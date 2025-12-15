use rust_engine::core::config::ConfigContainer;
use rust_engine::core::{TextureFormat, TextureHandle, TextureManager};

#[test]
fn texture_manager_load_and_get() {
    let config = ConfigContainer::new("conf/config.toml");
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
    let config = ConfigContainer::new("conf/config.toml");
    let mut mgr = TextureManager::new(config.get_config());

    let h1 = mgr.load("assets/tex1.png").unwrap();
    let h2 = mgr.load("assets/tex1.png").unwrap();

    // 同じパスをロードしたら同じハンドルが返る（キャッシュ）
    assert_eq!(h1, h2);
    assert_eq!(mgr.loaded_count(), 1);
}

#[test]
fn texture_manager_unload() {
    let config = ConfigContainer::new("conf/config.toml");
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

    let config = ConfigContainer::new("conf/config.toml");
    let mgr = TextureManager::new(config.get_config());
    assert!(mgr.get(&invalid).is_none());
}

#[test]
fn texture_manager_multiple_textures() {
    let config = ConfigContainer::new("conf/config.toml");
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
