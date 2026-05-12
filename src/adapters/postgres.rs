use crate::core::adapter::{Adapter, Row, Column};
use anyhow::Result;
use async_trait::async_trait;
use sqlx::postgres::PgPool;
use sqlx::{Row as SqlxRow, Column as SqlxColumn};

#[allow(dead_code)]
pub struct PostgresAdapter {
    pool: Option<PgPool>,
}

impl PostgresAdapter {
    pub fn new() -> Self {
        Self { pool: None }
    }
}

#[async_trait]
impl Adapter for PostgresAdapter {
    async fn connect(&mut self, connection_string: &str) -> Result<()> {
        let pool = PgPool::connect(connection_string).await?;
        self.pool = Some(pool);
        Ok(())
    }

    async fn execute(&self, query: &str) -> Result<()> {
        let pool = self.pool.as_ref().ok_or_else(|| anyhow::anyhow!("Not connected"))?;
        sqlx::query(query).execute(pool).await?;
        Ok(())
    }

    async fn query(&self, query: &str) -> Result<Vec<Row>> {
        let pool = self.pool.as_ref().ok_or_else(|| anyhow::anyhow!("Not connected"))?;
        let rows = sqlx::query(query).fetch_all(pool).await?;
        
        let mut results = Vec::new();
        for row in rows {
            let mut columns = Vec::new();
            for col in row.columns() {
                let name = col.name().to_string();
                let value: String = row.try_get(col.ordinal()).unwrap_or_else(|_| "null".to_string());
                columns.push(Column { name, value });
            }
            results.push(Row { columns });
        }
        
        Ok(results)
    }
}
