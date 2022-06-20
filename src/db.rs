use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool, Result};

pub struct Db {
    pool: PgPool,
}

#[derive(Debug, Serialize)]
pub struct Data1 {
    pub id: i32,
    pub name: String,
    pub value: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertData1Params {
    name: String,
    value: i32,
}

impl Db {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new().max_connections(5).connect(database_url).await?;
        Ok(Db { pool })
    }

    pub async fn insert_data1(&self, params: InsertData1Params) -> Result<i32> {
        let res = sqlx::query!("insert into data1 (name, value) values ($1, $2) returning id", params.name, params.value)
            .fetch_one(&self.pool)
            .await?;
        Ok(res.id)
    }

    pub async fn get_all_data1(&self) -> Result<Vec<Data1>> {
        sqlx::query_as!(Data1, "select * from data1 order by created_at desc").fetch_all(&self.pool).await
    }

    pub async fn get_data1(&self, id: i32) -> Result<Option<Data1>> {
        sqlx::query_as!(Data1, "select * from data1 where id=$1", id).fetch_optional(&self.pool).await
    }

    pub async fn delete_data1(&self, id: i32) -> Result<u64> {
        Ok(sqlx::query!("delete from data1 where id=$1", id).execute(&self.pool).await?.rows_affected())
    }

    pub async fn update_data1(&self, id: i32, value: i32) -> Result<u64> {
        Ok(sqlx::query!("update data1 set value=$1, updated_at=now() where id=$2", value, id)
            .execute(&self.pool)
            .await?
            .rows_affected())
    }
}
