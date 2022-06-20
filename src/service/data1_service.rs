use std::sync::Arc;

use crate::{db::Db, error::AppError};

pub struct Data1Service {
    db: Arc<Db>,
}

impl Data1Service {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }

    pub async fn process(&self, id: i32) -> crate::Result<i32> {
        let data1 = self.db.get_data1(id).await?.ok_or(AppError::NotFound)?;
        let new_value  = data1.value + 1;
        self.db.update_data1(id, new_value).await?;
        Ok(new_value)
    }
}
