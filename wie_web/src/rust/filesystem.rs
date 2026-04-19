use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};
use core::cmp::min;

use hashbrown::HashMap;
use js_sys::Uint8Array;
use spin::Mutex;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

use wie_backend::Filesystem;

#[wasm_bindgen(module = "/src/ts/filesystem.ts")]
extern "C" {
    type IndexedDBFilesystem;

    #[wasm_bindgen(static_method_of = IndexedDBFilesystem)]
    async fn open() -> IndexedDBFilesystem;

    #[wasm_bindgen(method)]
    async fn load_all(this: &IndexedDBFilesystem) -> js_sys::Array;

    #[wasm_bindgen(method)]
    async fn set(this: &IndexedDBFilesystem, aid: &str, path: &str, data: Uint8Array);

    #[wasm_bindgen(method)]
    async fn delete(this: &IndexedDBFilesystem, aid: &str, path: &str);
}

unsafe impl Sync for IndexedDBFilesystem {}
unsafe impl Send for IndexedDBFilesystem {}

pub struct WebFilesystem {
    db: IndexedDBFilesystem,
    files: Mutex<HashMap<(String, String), Vec<u8>>>,
}

impl WebFilesystem {
    pub async fn new() -> Self {
        let db = IndexedDBFilesystem::open().await;
        let entries = db.load_all().await;

        let mut files = HashMap::new();
        for entry in entries.iter() {
            let row: js_sys::Array = entry.into();
            let Some(aid) = row.get(0).as_string() else { continue };
            let Some(path) = row.get(1).as_string() else { continue };
            let data: Uint8Array = row.get(2).into();
            files.insert((aid, path), data.to_vec());
        }

        Self { db, files: Mutex::new(files) }
    }

    fn persist(&self, aid: String, path: String, data: Vec<u8>) {
        let db: IndexedDBFilesystem = self.db.clone().into();
        spawn_local(async move {
            let array = Uint8Array::from(data.as_slice());
            db.set(&aid, &path, array).await;
        });
    }

}

#[async_trait::async_trait]
impl Filesystem for WebFilesystem {
    async fn exists(&self, aid: &str, path: &str) -> bool {
        self.files.lock().contains_key(&(aid.to_string(), path.to_string()))
    }

    async fn size(&self, aid: &str, path: &str) -> Option<usize> {
        self.files.lock().get(&(aid.to_string(), path.to_string())).map(|v| v.len())
    }

    async fn read(&self, aid: &str, path: &str, offset: usize, count: usize, buf: &mut [u8]) -> Option<usize> {
        let files = self.files.lock();
        let data = files.get(&(aid.to_string(), path.to_string()))?;

        if offset >= data.len() {
            return Some(0);
        }

        let size_to_read = min(count, data.len() - offset);
        buf[..size_to_read].copy_from_slice(&data[offset..offset + size_to_read]);
        Some(size_to_read)
    }

    async fn write(&self, aid: &str, path: &str, offset: usize, data: &[u8]) -> usize {
        let snapshot = {
            let mut files = self.files.lock();
            let file = files.entry((aid.to_string(), path.to_string())).or_default();
            if file.len() < offset + data.len() {
                file.resize(offset + data.len(), 0);
            }
            file[offset..offset + data.len()].copy_from_slice(data);
            file.clone()
        };

        self.persist(aid.to_string(), path.to_string(), snapshot);
        data.len()
    }

    async fn truncate(&self, aid: &str, path: &str, len: usize) {
        let snapshot = {
            let mut files = self.files.lock();
            let file = files.entry((aid.to_string(), path.to_string())).or_default();
            file.resize(len, 0);
            file.clone()
        };

        self.persist(aid.to_string(), path.to_string(), snapshot);
    }
}
