use anyhow::{Context, Result};
use sqlx::{
    ConnectOptions, PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};
use std::str::FromStr;
use std::time::Duration;

pub mod deposit_processed;
pub mod event_chain;
mod instance;
pub mod receiver_usdt;
mod receiver_usdt_subjective_pre_entitle;
mod types;

pub use receiver_usdt_subjective_pre_entitle::subjective_pre_entitle_stats;

pub use instance::{ResolvedStream, ensure_instance_config, ensure_schema_version};

#[derive(Clone)]
pub struct Db {
    pub pool: PgPool,
}

impl Db {
    pub async fn connect(database_url: &str, max_connections: u32) -> Result<Self> {
        let opts = PgConnectOptions::from_str(database_url)
            .context("parse DATABASE_URL")?
            .log_statements(tracing::log::LevelFilter::Trace)
            .log_slow_statements(tracing::log::LevelFilter::Warn, Duration::from_millis(200));

        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect_with(opts)
            .await
            .context("connect to database")?;

        Ok(Self { pool })
    }
}
