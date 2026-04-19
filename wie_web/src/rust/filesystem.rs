use alloc::boxed::Box;
use core::cmp::min;

use js_sys::{Array, Uint8Array};
use wasm_bindgen::JsValue;

use wie_backend::Filesystem;

use crate::indexed_db_store::Store;

const DB_NAME: &str = "wie_filesystem";
const STORE_NAME: &str = "files";

fn make_key(aid: &str, path: &str) -> JsValue {
    Array::of2(&JsValue::from_str(aid), &JsValue::from_str(path)).into()
}

pub struct WebFilesystem {
    store: Store,
}

impl WebFilesystem {
    pub async fn new() -> Self {
        Self {
            store: Store::open(DB_NAME, STORE_NAME).await,
        }
    }
}

#[async_trait::async_trait]
impl Filesystem for WebFilesystem {
    async fn exists(&self, aid: &str, path: &str) -> bool {
        self.store.get(make_key(aid, path)).await.is_some()
    }

    async fn size(&self, aid: &str, path: &str) -> Option<usize> {
        self.store.get(make_key(aid, path)).await.map(|a| a.length() as usize)
    }

    async fn read(&self, aid: &str, path: &str, offset: usize, count: usize, buf: &mut [u8]) -> Option<usize> {
        let array = self.store.get(make_key(aid, path)).await?;
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
        let key = make_key(aid, path);
        let existing = self.store.get(key.clone()).await;
        let existing_len = existing.as_ref().map(|a| a.length() as usize).unwrap_or(0);
        let new_len = core::cmp::max(existing_len, offset + data.len());

        let next = Uint8Array::new_with_length(new_len as u32);
        if let Some(existing) = existing {
            next.set(&existing, 0);
        }
        next.subarray(offset as u32, (offset + data.len()) as u32).copy_from(data);

        self.store.set(key, next).await;
        data.len()
    }

    async fn truncate(&self, aid: &str, path: &str, len: usize) {
        let key = make_key(aid, path);
        let existing = self.store.get(key.clone()).await;
        let next = Uint8Array::new_with_length(len as u32);
        if let Some(existing) = existing {
            let copy_len = min(existing.length() as usize, len) as u32;
            next.set(&existing.subarray(0, copy_len), 0);
        }

        self.store.set(key, next).await;
    }
}
