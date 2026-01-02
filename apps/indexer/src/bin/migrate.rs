use anyhow::{Context, Result};
use sqlx::Postgres;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[derive(Debug)]
struct Args {
    notify_pgrst: bool,
    pgrst_channel: String,
    pgrst_payload: String,
}

fn parse_args() -> Result<Args> {
    let mut notify_pgrst = true;
    let mut pgrst_channel = "pgrst".to_string();
    let mut pgrst_payload = "reload schema".to_string();

    let mut it = env::args().skip(1);
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--no-notify-pgrst" => notify_pgrst = false,
            "--pgrst-channel" => {
                pgrst_channel = it.next().context("--pgrst-channel requires a value")?;
            }
            "--pgrst-payload" => {
                pgrst_payload = it.next().context("--pgrst-payload requires a value")?;
            }
            _ => anyhow::bail!("unknown argument: {arg}"),
        }
    }

    Ok(Args {
        notify_pgrst,
        pgrst_channel,
        pgrst_payload,
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = parse_args()?;
    let database_url = env::var("DATABASE_URL").context("DATABASE_URL must be set")?;

    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&database_url)
        .await
        .context("connect to database")?;

    // Apply migrations.
    sqlx::migrate!("./db/migrations")
        .run(&pool)
        .await
        .context("run migrations")?;

    if args.notify_pgrst {
        sqlx::query("select pg_notify($1, $2)")
            .bind(&args.pgrst_channel)
            .bind(&args.pgrst_payload)
            .execute(&pool)
            .await
            .with_context(|| {
                format!(
                    "notify PostgREST channel={} payload={}",
                    args.pgrst_channel, args.pgrst_payload
                )
            })?;
    }

    // Helpful in logs/CI: print the schema migration version.
    let version: i64 = sqlx::query_scalar::<Postgres, i64>(
        "select coalesce(max(version), 0) from _sqlx_migrations",
    )
    .fetch_one(&pool)
    .await
    .context("read migration version")?;

    println!("migrations applied, schema version={version}");

    Ok(())
}
