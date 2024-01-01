use alloc::{boxed::Box, vec::Vec};

use wie_backend::RecordId;

pub struct DatabaseRepository {}

impl DatabaseRepository {
    pub fn new(_app_id: &str) -> Self {
        Self {}
    }
}

impl wie_backend::DatabaseRepository for DatabaseRepository {
    fn open(&self, _name: &str) -> Box<dyn wie_backend::Database> {
        Box::new(Database::new().unwrap())
    }
}

pub struct Database {}

impl Database {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {})
    }
}

impl wie_backend::Database for Database {
    fn add(&mut self, _data: &[u8]) -> RecordId {
        0
    }

    fn get(&self, _id: RecordId) -> Option<Vec<u8>> {
        None
    }

    fn set(&mut self, _id: RecordId, _data: &[u8]) -> bool {
        true
    }

    fn delete(&mut self, _id: RecordId) -> bool {
        true
    }

    fn get_record_ids(&self) -> Vec<RecordId> {
        Vec::new()
    }
}
