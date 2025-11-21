use std::collections::HashMap;
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
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            path_cache: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn load(&mut self, path: &str) -> TextureHandle {
        if let Some(handle) = self.path_cache.get(path) {
            return *handle;
        }
        let id = self.next_id;
        self.next_id += 1;
        let data = self._load_impl(path);
        self.textures.insert(id, data);
        let handle = TextureHandle { id };
        self.path_cache.insert(path.to_string(), handle);
        handle
    }

    fn _load_impl(&mut self, _path: &str) -> TextureData {
        // ここで実際のファイル読み込みとデコードを行う
        // 仮の実装として、固定サイズとフォーマットのテクスチャを返す
        TextureData {
            width: 256,
            height: 256,
            format: TextureFormat::Rgba8,
            data: vec![255; 256 * 256 * 4], // 白いテクスチャ
        }
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
