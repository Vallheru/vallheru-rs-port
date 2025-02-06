
use vallheru::config::Config;
use vallheru::web::serve;
use anyhow::Context;
use clap::Parser;
use sqlx::postgres::PgPoolOptions;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = Config::parse();

    let db = PgPoolOptions::new()
        .max_connections(cfg.database_max_connections)
        .connect(&cfg.database_url)
        .await
        .context("could not connect to database")?;

    serve(cfg, db).await?;

    Ok(())
}
