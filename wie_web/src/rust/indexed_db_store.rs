use alloc::{borrow::ToOwned, vec::Vec};

use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

use crate::util::run_js_future;

#[wasm_bindgen(module = "/src/ts/indexed_db_store.ts")]
extern "C" {
    type IndexedDBStore;

    #[wasm_bindgen(static_method_of = IndexedDBStore)]
    async fn open(db_name: &str, store_name: &str) -> IndexedDBStore;

    #[wasm_bindgen(method)]
    async fn get_all_keys(this: &IndexedDBStore) -> js_sys::Array;

    #[wasm_bindgen(method)]
    async fn get(this: &IndexedDBStore, key: &JsValue) -> JsValue;

    #[wasm_bindgen(method)]
    async fn set(this: &IndexedDBStore, key: &JsValue, data: Uint8Array);

    #[wasm_bindgen(method)]
    async fn delete(this: &IndexedDBStore, key: &JsValue);
}

unsafe impl Sync for IndexedDBStore {}
unsafe impl Send for IndexedDBStore {}

pub struct Store {
    js: IndexedDBStore,
}

impl Store {
    pub async fn open(db_name: &str, store_name: &str) -> Self {
        let db_name = db_name.to_owned();
        let store_name = store_name.to_owned();
        let js = run_js_future(async move { IndexedDBStore::open(&db_name, &store_name).await }).await;
        Self { js }
    }

    fn clone_js(&self) -> IndexedDBStore {
        self.js.clone().into()
    }

    pub async fn get_all_keys(&self) -> Vec<JsValue> {
        let js = self.clone_js();
        let keys = run_js_future(async move { js.get_all_keys().await }).await;
        keys.iter().collect()
    }

    pub async fn get(&self, key: JsValue) -> Option<Uint8Array> {
        let js = self.clone_js();
        let data = run_js_future(async move { js.get(&key).await }).await;
        if data.is_undefined() { None } else { Some(data.into()) }
    }

    pub async fn set(&self, key: JsValue, data: Uint8Array) {
        let js = self.clone_js();
        run_js_future(async move { js.set(&key, data).await }).await;
    }

    pub async fn delete(&self, key: JsValue) {
        let js = self.clone_js();
        run_js_future(async move { js.delete(&key).await }).await;
    }
}
