use surrealdb::{Surreal, engine::local::RocksDb};

pub struct Database {
    pub client: Surreal<surrealdb::engine::local::Db>,
}

impl Database {
    pub async fn connect() -> anyhow::Result<Self> {
        let client = Surreal::new::<RocksDb>("data/planner.db").await?;

        client.use_ns("planner").use_db("planner").await?;

        tracing::info!("SurrealDB 연결 완료 (RocksDB)");

        Ok(Self { client })
    }
}
