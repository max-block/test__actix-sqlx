use std::sync::Arc;

use crate::{db::Db, service::Data1Service};

pub struct App {
    pub db: Arc<Db>,
    pub data1_service: Data1Service,
}

impl App {
    pub async fn new(database_url: &str) -> Self {
        let db = Db::new(database_url).await.expect("can't init database");
        let db = Arc::new(db);
        let data1_service = Data1Service::new(db.clone());
        Self { db, data1_service }
    }
}
