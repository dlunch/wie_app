use alloc::{boxed::Box, format, string::ToString, vec::Vec};

use js_sys::{Int32Array, Uint8Array};
use tokio::sync::oneshot;
use wasm_bindgen::prelude::*;

use wasm_bindgen_futures::spawn_local;
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
    async fn open(&self, name: &str, app_id: &str) -> Box<dyn wie_backend::Database> {
        Box::new(Database::new(name, app_id).await.unwrap())
    }
}

pub struct Database {
    db: IndexedDBStore,
}
impl Database {
    pub async fn new(name: &str, app_id: &str) -> anyhow::Result<Self> {
        let (tx, rx) = oneshot::channel();

        let db_name = format!("wie_{app_id}");
        let name = name.to_string();
        spawn_local(async move {
            // wasm async method is not Send, so we have to work around it
            let db = IndexedDBStore::open(&db_name, &name).await;

            tx.send(db).unwrap_or_else(|_| panic!())
        });

        let db = rx.await.unwrap();

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
        let (tx, rx) = oneshot::channel();

        let db: IndexedDBStore = self.db.clone().into();
        spawn_local(async move {
            let data = db.get(id).await;

            let result = if data.is_null() {
                None
            } else {
                let array: Uint8Array = data.into();
                Some(array.to_vec())
            };

            tx.send(result).unwrap();
        });

        rx.await.unwrap()
    }

    async fn set(&mut self, id: RecordId, data: &[u8]) -> bool {
        let (tx, rx) = oneshot::channel();

        let db: IndexedDBStore = self.db.clone().into();
        let data = data.to_vec();
        spawn_local(async move {
            db.set(id, &data).await;
            tx.send(true).unwrap();
        });

        rx.await.unwrap()
    }

    async fn delete(&mut self, id: RecordId) -> bool {
        let (tx, rx) = oneshot::channel();

        let db: IndexedDBStore = self.db.clone().into();
        spawn_local(async move {
            db.delete(id).await;
            tx.send(true).unwrap();
        });

        rx.await.unwrap()
    }

    async fn get_record_ids(&self) -> Vec<RecordId> {
        let (tx, rx) = oneshot::channel();

        let db: IndexedDBStore = self.db.clone().into();
        spawn_local(async move {
            let ids = db.get_record_ids().await;
            let ids_vec = ids.to_vec().into_iter().map(|id| id as _).collect();
            tx.send(ids_vec).unwrap();
        });

        rx.await.unwrap()
    }
}
