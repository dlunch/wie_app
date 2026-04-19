use alloc::{boxed::Box, string::ToString};
use core::cmp::min;

use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

use wie_backend::Filesystem;

use crate::util::run_js_future;

#[wasm_bindgen(module = "/src/ts/filesystem.ts")]
extern "C" {
    type IndexedDBFilesystem;

    #[wasm_bindgen(static_method_of = IndexedDBFilesystem)]
    async fn open() -> IndexedDBFilesystem;

    #[wasm_bindgen(method)]
    async fn exists(this: &IndexedDBFilesystem, aid: &str, path: &str) -> JsValue; // bool

    #[wasm_bindgen(method)]
    async fn get(this: &IndexedDBFilesystem, aid: &str, path: &str) -> JsValue; // Uint8Array | undefined

    #[wasm_bindgen(method)]
    async fn write(this: &IndexedDBFilesystem, aid: &str, path: &str, offset: usize, data: Uint8Array) -> JsValue; // number

    #[wasm_bindgen(method)]
    async fn truncate(this: &IndexedDBFilesystem, aid: &str, path: &str, length: usize);
}

unsafe impl Sync for IndexedDBFilesystem {}
unsafe impl Send for IndexedDBFilesystem {}

pub struct WebFilesystem {
    db: IndexedDBFilesystem,
}

impl WebFilesystem {
    pub async fn new() -> Self {
        let db = run_js_future(async { IndexedDBFilesystem::open().await }).await.into_inner();
        Self { db }
    }

    fn db(&self) -> IndexedDBFilesystem {
        self.db.clone().into()
    }
}

#[async_trait::async_trait]
impl Filesystem for WebFilesystem {
    async fn exists(&self, aid: &str, path: &str) -> bool {
        let db = self.db();
        let aid = aid.to_string();
        let path = path.to_string();
        run_js_future(async move { db.exists(&aid, &path).await })
            .await
            .into_inner()
            .as_bool()
            .unwrap_or(false)
    }

    async fn size(&self, aid: &str, path: &str) -> Option<usize> {
        let db = self.db();
        let aid = aid.to_string();
        let path = path.to_string();
        let data = run_js_future(async move { db.get(&aid, &path).await }).await.into_inner();

        if data.is_undefined() {
            None
        } else {
            let array: Uint8Array = data.into();
            Some(array.length() as usize)
        }
    }

    async fn read(&self, aid: &str, path: &str, offset: usize, count: usize, buf: &mut [u8]) -> Option<usize> {
        let db = self.db();
        let aid_owned = aid.to_string();
        let path_owned = path.to_string();
        let data = run_js_future(async move { db.get(&aid_owned, &path_owned).await })
            .await
            .into_inner();

        if data.is_undefined() {
            return None;
        }

        let array: Uint8Array = data.into();
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
        let db = self.db();
        let aid = aid.to_string();
        let path = path.to_string();
        let array = Uint8Array::from(data);
        let len = data.len();
        let result = run_js_future(async move { db.write(&aid, &path, offset, array).await })
            .await
            .into_inner();

        result.as_f64().map(|v| v as usize).unwrap_or(len)
    }

    async fn truncate(&self, aid: &str, path: &str, len: usize) {
        let db = self.db();
        let aid = aid.to_string();
        let path = path.to_string();
        run_js_future(async move { db.truncate(&aid, &path, len).await }).await;
    }
}
