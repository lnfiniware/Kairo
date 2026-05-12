use crate::core::adapter::{Adapter, Row};
use anyhow::Result;
use async_trait::async_trait;

#[allow(dead_code)]
pub struct PostgresAdapter;

#[async_trait]
impl Adapter for PostgresAdapter {
    #[allow(dead_code)]
    async fn connect(&mut self, _connection_string: &str) -> Result<()> {
        unimplemented!("PostgreSQL adapter coming soon")
    }

    #[allow(dead_code)]
    async fn execute(&self, _query: &str) -> Result<()> {
        unimplemented!()
    }

    async fn query(&self, _query: &str) -> Result<Vec<Row>> {
        unimplemented!()
    }
}
