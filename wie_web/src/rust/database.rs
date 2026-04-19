use alloc::{
    boxed::Box,
    format,
    string::{String, ToString},
    vec::Vec,
};

use js_sys::{Int32Array, Uint8Array};
use wasm_bindgen::prelude::*;

use wie_backend::{RecordId, System};

use crate::util::run_js_future;

#[wasm_bindgen(module = "/src/ts/database.ts")]
extern "C" {
    type IndexedDBStore;

    #[wasm_bindgen(static_method_of = IndexedDBStore)]
    async fn open(store_name: &str, key_prefix: &str) -> IndexedDBStore;

    #[wasm_bindgen(static_method_of = IndexedDBStore)]
    async fn exists(store_name: &str, key_prefix: &str) -> JsValue; // bool

    #[wasm_bindgen(method)]
    async fn get_record_ids(this: &IndexedDBStore) -> Int32Array; // Vec<RecordId>

    #[wasm_bindgen(method)]
    async fn set(this: &IndexedDBStore, id: RecordId, data: Uint8Array);

    #[wasm_bindgen(method)]
    async fn get(this: &IndexedDBStore, id: RecordId) -> JsValue; // Option<Vec<u8>>

    #[wasm_bindgen(method)]
    async fn delete(this: &IndexedDBStore, id: RecordId);
}

unsafe impl Sync for IndexedDBStore {}
unsafe impl Send for IndexedDBStore {}

pub struct DatabaseRepository {}

impl DatabaseRepository {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl wie_backend::DatabaseRepository for DatabaseRepository {
    async fn open(&self, _system: &System, name: &str, app_id: &str) -> Box<dyn wie_backend::Database> {
        let db_name = format!("wie_{app_id}");
        let key_prefix = name.to_string();

        Box::new(Database::new(db_name, key_prefix).await.unwrap())
    }

    async fn exists(&self, _system: &System, name: &str, app_id: &str) -> bool {
        let db_name = format!("wie_{app_id}");
        let key_prefix = name.to_string();

        run_js_future(async move { IndexedDBStore::exists(&db_name, &key_prefix).await })
            .await
            .into_inner()
            .as_bool()
            .unwrap()
    }
}

pub struct Database {
    db: IndexedDBStore,
}

impl Database {
    pub async fn new(db_name: String, key_prefix: String) -> anyhow::Result<Self> {
        let db = run_js_future(async move { IndexedDBStore::open(&db_name, &key_prefix).await })
            .await
            .into_inner();

        Ok(Self { db })
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
        let db: IndexedDBStore = self.db.clone().into();
        let data = run_js_future(async move { db.get(id).await }).await.into_inner();

        if data.is_undefined() {
            None
        } else {
            let array: Uint8Array = data.into();
            Some(array.to_vec())
        }
    }

    async fn set(&mut self, id: RecordId, data: &[u8]) -> bool {
        let db: IndexedDBStore = self.db.clone().into();
        let data = Uint8Array::from(data);
        run_js_future(async move { db.set(id, data).await }).await;

        true
    }

    async fn delete(&mut self, id: RecordId) -> bool {
        let db: IndexedDBStore = self.db.clone().into();
        run_js_future(async move { db.delete(id).await }).await;

        true
    }

    async fn get_record_ids(&self) -> Vec<RecordId> {
        let db: IndexedDBStore = self.db.clone().into();
        let ids = run_js_future(async move { db.get_record_ids().await }).await.into_inner();

        ids.to_vec().into_iter().map(|id| id as RecordId).collect()
    }
}
