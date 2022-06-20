use std::sync::Arc;

use crate::db::Db;

pub struct Data1Service {
    db: Arc<Db>,
}

impl Data1Service {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }
}
