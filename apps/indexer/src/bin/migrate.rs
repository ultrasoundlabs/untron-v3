use anyhow::{Context, Result};
use clap::{ArgAction, Parser};
use sqlx::Postgres;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[derive(Debug, Parser)]
#[command(name = "migrate", disable_help_subcommand = true)]
struct Args {
    #[arg(long = "no-notify-pgrst", action = ArgAction::SetFalse, default_value_t = true)]
    notify_pgrst: bool,

    #[arg(long, default_value = "pgrst")]
    pgrst_channel: String,

    #[arg(long, default_value = "reload schema")]
    pgrst_payload: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
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
