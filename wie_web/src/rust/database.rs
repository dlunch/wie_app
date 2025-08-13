use alloc::{boxed::Box, vec::Vec};

use js_sys::Int32Array;
use wasm_bindgen::prelude::*;

use wie_backend::RecordId;

#[wasm_bindgen(module = "database.ts")]
extern "C" {
    type IndexedDBStore;

    #[wasm_bindgen(static_method_of = IndexedDBStore)]
    async fn open(db_name: &str, store_name: &str) -> IndexedDBStore;

    #[wasm_bindgen(method)]
    async fn get_record_ids(this: &IndexedDBStore) -> Int32Array; // Vec<RecordId>

    #[wasm_bindgen(method)]
    async fn set(this: &IndexedDBStore, id: RecordId, data: &[u8]);

    #[wasm_bindgen(method)]
    async fn get(this: &IndexedDBStore, id: RecordId) -> JsValue; // Option<Vec<u8>>

    #[wasm_bindgen(method)]
    async fn delete(this: &IndexedDBStore, id: RecordId);
}

pub struct DatabaseRepository {}

impl DatabaseRepository {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl wie_backend::DatabaseRepository for DatabaseRepository {
    async fn open(&self, _name: &str, _app_id: &str) -> Box<dyn wie_backend::Database> {
        Box::new(Database::new().unwrap())
    }
}

pub struct Database {}

impl Database {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {})
    }
}

#[async_trait::async_trait]
impl wie_backend::Database for Database {
    async fn add(&mut self, _data: &[u8]) -> RecordId {
        1
    }

    async fn next_id(&self) -> RecordId {
        1
    }

    async fn get(&self, _id: RecordId) -> Option<Vec<u8>> {
        None
    }

    async fn set(&mut self, _id: RecordId, _data: &[u8]) -> bool {
        true
    }

    async fn delete(&mut self, _id: RecordId) -> bool {
        true
    }

    async fn get_record_ids(&self) -> Vec<RecordId> {
        Vec::new()
    }
}
