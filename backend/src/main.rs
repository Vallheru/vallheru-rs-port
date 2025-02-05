mod config;
use config::Config;

mod web;
use web::serve;

mod repository;
mod controller;
mod player_state;

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
