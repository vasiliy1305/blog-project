mod application;
mod data;
mod domain;
mod infrastructure;
mod presentation;
mod server;

use crate::infrastructure::{
    config::Config,
    database::{create_pool, run_migrations},
};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    infrastructure::logging::init();

    let config = Config::from_env()?;

    tracing::info!("Connecting to PostgreSQL");

    let pool = create_pool(&config.database_url).await?;

    tracing::info!("Running database migrations");

    run_migrations(&pool).await?;

    server::run_http_server(config, pool).await?;

    Ok(())
}
