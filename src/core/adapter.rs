use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Adapter {
    async fn connect(&mut self, connection_string: &str) -> Result<()>;
    async fn execute(&self, query: &str) -> Result<()>;
    async fn query(&self, query: &str) -> Result<Vec<Row>>;
}

pub struct Row {
    pub columns: Vec<Column>,
}

pub struct Column {
    pub name: String,
    pub value: String,
}
