use alloc::{
    boxed::Box,
    format,
    string::{String, ToString},
    vec::Vec,
};

use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

use wie_backend::{RecordId, System};

use crate::util::run_js_future;

#[wasm_bindgen(module = "/src/ts/indexed_db_store.ts")]
extern "C" {
    type IndexedDBStore;

    #[wasm_bindgen(static_method_of = IndexedDBStore)]
    async fn open(db_name: &str, store_name: &str) -> IndexedDBStore;

    #[wasm_bindgen(method)]
    async fn get_all_keys(this: &IndexedDBStore) -> js_sys::Array;

    #[wasm_bindgen(method)]
    async fn get(this: &IndexedDBStore, key: &str) -> JsValue;

    #[wasm_bindgen(method)]
    async fn set(this: &IndexedDBStore, key: &str, data: Uint8Array);

    #[wasm_bindgen(method)]
    async fn delete(this: &IndexedDBStore, key: &str);
}

unsafe impl Sync for IndexedDBStore {}
unsafe impl Send for IndexedDBStore {}

fn db_name(app_id: &str) -> String {
    format!("wie_{app_id}")
}

async fn open_store(app_id: &str) -> IndexedDBStore {
    let db_name = db_name(app_id);
    run_js_future(async move { IndexedDBStore::open(&db_name, &db_name).await })
        .await
        .into_inner()
}

pub struct DatabaseRepository {}

impl DatabaseRepository {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl wie_backend::DatabaseRepository for DatabaseRepository {
    async fn open(&self, _system: &System, name: &str, app_id: &str) -> Box<dyn wie_backend::Database> {
        let store = open_store(app_id).await;
        Box::new(Database { store, key_prefix: name.to_string() })
    }

    async fn exists(&self, _system: &System, name: &str, app_id: &str) -> bool {
        let store = open_store(app_id).await;
        let prefix = name.to_string();
        let keys = run_js_future(async move { store.get_all_keys().await }).await.into_inner();

        keys.iter().any(|k| k.as_string().map(|s| s.starts_with(&prefix)).unwrap_or(false))
    }
}

pub struct Database {
    store: IndexedDBStore,
    key_prefix: String,
}

impl Database {
    fn record_key(&self, id: RecordId) -> String {
        format!("{}{}", self.key_prefix, id)
    }
}

#[async_trait::async_trait]
impl wie_backend::Database for Database {
    async fn add(&mut self, data: &[u8]) -> RecordId {
        let id = self.next_id().await;
        self.set(id, data).await;

        id
    }

    async fn next_id(&self) -> RecordId {
        let ids = self.get_record_ids().await;

        ids.iter().max().map_or(1, |&id| id + 1)
    }

    async fn get(&self, id: RecordId) -> Option<Vec<u8>> {
        let store: IndexedDBStore = self.store.clone().into();
        let key = self.record_key(id);
        let data = run_js_future(async move { store.get(&key).await }).await.into_inner();

        if data.is_undefined() {
            None
        } else {
            let array: Uint8Array = data.into();
            Some(array.to_vec())
        }
    }

    async fn set(&mut self, id: RecordId, data: &[u8]) -> bool {
        let store: IndexedDBStore = self.store.clone().into();
        let key = self.record_key(id);
        let data = Uint8Array::from(data);
        run_js_future(async move { store.set(&key, data).await }).await;

        true
    }

    async fn delete(&mut self, id: RecordId) -> bool {
        let store: IndexedDBStore = self.store.clone().into();
        let key = self.record_key(id);
        run_js_future(async move { store.delete(&key).await }).await;

        true
    }

    async fn get_record_ids(&self) -> Vec<RecordId> {
        let store: IndexedDBStore = self.store.clone().into();
        let keys = run_js_future(async move { store.get_all_keys().await }).await.into_inner();

        let prefix = &self.key_prefix;
        keys.iter()
            .filter_map(|k| k.as_string())
            .filter_map(|s| s.strip_prefix(prefix.as_str()).and_then(|tail| tail.parse::<RecordId>().ok()))
            .collect()
    }
}
