use alloc::{
    boxed::Box,
    format,
    string::{String, ToString},
};
use core::cmp::min;

use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

use wie_backend::Filesystem;

use crate::util::run_js_future;

#[wasm_bindgen(module = "/src/ts/indexed_db_store.ts")]
extern "C" {
    type IndexedDBStore;

    #[wasm_bindgen(static_method_of = IndexedDBStore)]
    async fn open(db_name: &str, store_name: &str) -> IndexedDBStore;

    #[wasm_bindgen(method)]
    async fn get(this: &IndexedDBStore, key: &str) -> JsValue;

    #[wasm_bindgen(method)]
    async fn set(this: &IndexedDBStore, key: &str, data: Uint8Array);
}

unsafe impl Sync for IndexedDBStore {}
unsafe impl Send for IndexedDBStore {}

const DB_NAME: &str = "wie_filesystem";
const STORE_NAME: &str = "files";

fn make_key(aid: &str, path: &str) -> String {
    format!("{aid}\0{path}")
}

pub struct WebFilesystem {
    store: IndexedDBStore,
}

impl WebFilesystem {
    pub async fn new() -> Self {
        let store = run_js_future(async { IndexedDBStore::open(DB_NAME, STORE_NAME).await })
            .await
            .into_inner();
        Self { store }
    }

    async fn load(&self, aid: &str, path: &str) -> Option<Uint8Array> {
        let store: IndexedDBStore = self.store.clone().into();
        let key = make_key(aid, path);
        let data = run_js_future(async move { store.get(&key).await }).await.into_inner();

        if data.is_undefined() { None } else { Some(data.into()) }
    }

    async fn store_blob(&self, aid: &str, path: &str, data: Uint8Array) {
        let store: IndexedDBStore = self.store.clone().into();
        let key = make_key(aid, path);
        run_js_future(async move { store.set(&key, data).await }).await;
    }
}

#[async_trait::async_trait]
impl Filesystem for WebFilesystem {
    async fn exists(&self, aid: &str, path: &str) -> bool {
        self.load(aid, path).await.is_some()
    }

    async fn size(&self, aid: &str, path: &str) -> Option<usize> {
        self.load(aid, path).await.map(|a| a.length() as usize)
    }

    async fn read(&self, aid: &str, path: &str, offset: usize, count: usize, buf: &mut [u8]) -> Option<usize> {
        let array = self.load(aid, path).await?;
        let size = array.length() as usize;

        if offset >= size {
            return Some(0);
        }

        let size_to_read = min(count, size - offset);
        array
            .subarray(offset as u32, (offset + size_to_read) as u32)
            .copy_to(&mut buf[..size_to_read]);
        Some(size_to_read)
    }

    async fn write(&self, aid: &str, path: &str, offset: usize, data: &[u8]) -> usize {
        let existing = self.load(aid, path).await;
        let existing_len = existing.as_ref().map(|a| a.length() as usize).unwrap_or(0);
        let new_len = core::cmp::max(existing_len, offset + data.len());

        let next = Uint8Array::new_with_length(new_len as u32);
        if let Some(existing) = existing {
            next.set(&existing, 0);
        }
        next.subarray(offset as u32, (offset + data.len()) as u32)
            .copy_from(data);

        self.store_blob(&aid.to_string(), &path.to_string(), next).await;
        data.len()
    }

    async fn truncate(&self, aid: &str, path: &str, len: usize) {
        let existing = self.load(aid, path).await;
        let next = Uint8Array::new_with_length(len as u32);
        if let Some(existing) = existing {
            let copy_len = min(existing.length() as usize, len) as u32;
            next.set(&existing.subarray(0, copy_len), 0);
        }

        self.store_blob(&aid.to_string(), &path.to_string(), next).await;
    }
}
