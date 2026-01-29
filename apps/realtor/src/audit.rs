use anyhow::{Context, Result};
use axum::http::HeaderMap;
use serde_json::Value;
use sqlx::{PgPool, postgres::PgPoolOptions, types::Json};
use uuid::Uuid;

#[derive(Clone)]
pub struct AuditDb {
    pool: PgPool,
}

#[derive(Debug, Clone)]
pub struct AuditContext {
    pub request_id: Option<Uuid>,
    pub principal_id: Option<String>,
    pub remote_ip: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Clone)]
pub struct WriteAction {
    pub request_id: Option<Uuid>,
    pub principal_id: Option<String>,
    pub remote_ip: Option<String>,
    pub user_agent: Option<String>,
    pub action: &'static str,
    pub method: &'static str,
    pub path: &'static str,
    pub status_code: u16,
    pub duration_ms: u64,
    pub error_kind: Option<&'static str>,
    pub error_message: Option<String>,
    pub request_body: Option<Value>,
    pub response_body: Option<Value>,
}

fn header_string(headers: &HeaderMap, name: &'static str) -> Option<String> {
    headers
        .get(name)
        .and_then(|v| v.to_str().ok())
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(str::to_string)
}

impl AuditContext {
    pub fn from_headers(headers: &HeaderMap) -> Self {
        let request_id =
            header_string(headers, "x-request-id").and_then(|v| Uuid::parse_str(v.as_str()).ok());
        let principal_id = header_string(headers, "x-untron-principal-id");
        let user_agent = header_string(headers, "user-agent");
        let remote_ip = header_string(headers, "x-forwarded-for")
            .and_then(|v| v.split(',').next().map(str::trim).map(str::to_string))
            .filter(|v| !v.is_empty())
            .or_else(|| header_string(headers, "x-real-ip"));

        Self {
            request_id,
            principal_id,
            remote_ip,
            user_agent,
        }
    }
}

impl AuditDb {
    pub async fn connect(database_url: &str, max_connections: u32) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(max_connections.max(1))
            .connect(database_url)
            .await
            .context("connect to realtor audit db")?;

        let exists: Option<String> =
            sqlx::query_scalar("select to_regclass('realtor.write_action')::text")
                .fetch_one(&pool)
                .await
                .context("check realtor.write_action exists")?;
        if exists.is_none() {
            anyhow::bail!(
                "missing table realtor.write_action (run apps/indexer DB migrations against this database)"
            );
        }

        Ok(Self { pool })
    }

    pub async fn insert_write_action(&self, a: WriteAction) -> Result<()> {
        sqlx::query(
            r#"
insert into realtor.write_action (
  request_id,
  principal_id,
  remote_ip,
  user_agent,
  action,
  method,
  path,
  status_code,
  duration_ms,
  error_kind,
  error_message,
  request_body,
  response_body
)
values ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13)
"#,
        )
        .bind(a.request_id)
        .bind(a.principal_id)
        .bind(a.remote_ip)
        .bind(a.user_agent)
        .bind(a.action)
        .bind(a.method)
        .bind(a.path)
        .bind(i32::from(a.status_code))
        .bind(i64::try_from(a.duration_ms).unwrap_or(i64::MAX))
        .bind(a.error_kind)
        .bind(a.error_message)
        .bind(a.request_body.map(Json))
        .bind(a.response_body.map(Json))
        .execute(&self.pool)
        .await
        .context("insert realtor.write_action")?;

        Ok(())
    }
}
