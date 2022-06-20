use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};

pub struct Db {
    pool: PgPool,
}

#[derive(Debug, Serialize)]
pub struct Data1 {
    id: i32,
    name: String,
    value: i32,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertData1Params {
    name: String,
    value: i32,
}

impl Db {
    pub async fn new(database_url: &str) -> sqlx::Result<Self> {
        let pool = PgPoolOptions::new().max_connections(5).connect(database_url).await?;
        Ok(Db { pool })
    }

    pub async fn insert_data1(&self, params: InsertData1Params) -> sqlx::Result<i32> {
        let res = sqlx::query!("insert into data1 (name, value) values ($1, $2) returning id", params.name, params.value)
            .fetch_one(&self.pool)
            .await?;
        Ok(res.id)
    }

    pub async fn get_all_data1(&self) -> sqlx::Result<Vec<Data1>> {
        sqlx::query_as!(Data1, "select * from data1 order by created_at desc").fetch_all(&self.pool).await
    }
}
