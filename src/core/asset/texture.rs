use crate::core::config::Config;
use image::GenericImageView;
use std::collections::HashMap;
use std::sync::Arc;

type TextureId = u32;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextureHandle {
    id: TextureId,
}

impl TextureHandle {
    pub fn invalid() -> Self {
        Self { id: 0 }
    }
    pub fn is_valid(&self) -> bool {
        self.id != 0
    }
    pub fn id(&self) -> TextureId {
        self.id
    }
}

#[derive(Debug, Clone)]
pub struct TextureData {
    pub width: u32,
    pub height: u32,
    pub format: TextureFormat,
    pub data: Vec<u8>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureFormat {
    Rgba8,
    Rgb8,
    Bgra8,
    // 他のフォーマットも追加可能
}

pub struct TextureManager {
    textures: HashMap<TextureId, TextureData>,
    path_cache: HashMap<String, TextureHandle>,
    next_id: TextureId,
    config: Arc<Config>,
}

impl TextureManager {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            textures: HashMap::new(),
            path_cache: HashMap::new(),
            next_id: 1,
            config: config,
        }
    }

    pub fn load(&mut self, path: &str) -> Result<TextureHandle, String> {
        if let Some(handle) = self.path_cache.get(path) {
            return Ok(*handle);
        }
        let id = self.next_id;
        self.next_id += 1;
        let full_path = self.config.get_texture_dir().unwrap() + path;
        let data = self._load_impl(&full_path)?;
        self.textures.insert(id, data);
        let handle = TextureHandle { id };
        self.path_cache.insert(path.to_string(), handle);
        Ok(handle)
    }

    fn _load_impl(&mut self, _path: &str) -> Result<TextureData, String> {
        // ここで実際のファイル読み込みとデコードを行う
        let img = match image::open(_path) {
            Ok(img) => img,
            Err(e) => {
                log::error!("Failed to load image: {}", e);
                return Err(e.to_string());
            }
        };
        let (width, height) = img.dimensions();
        let format = TextureFormat::Rgba8;
        let data = img.as_bytes().to_vec();
        Ok(TextureData {
            width,
            height,
            format,
            data,
        })
    }

    pub fn get(&self, handle: &TextureHandle) -> Option<&TextureData> {
        self.textures.get(&handle.id)
    }

    pub fn unload(&mut self, handle: TextureHandle) {
        if let Some(_) = self.textures.remove(&handle.id) {
            // 必要に応じて追加のクリーンアップ処理をここで行う
        }
    }

    pub fn loaded_count(&self) -> usize {
        self.textures.len()
    }
}
