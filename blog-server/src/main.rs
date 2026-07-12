mod application;
mod data;
mod domain;
mod infrastructure;
mod presentation;
mod server;

use std::sync::Arc;

use infrastructure::jwt::JwtService;

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

    let jwt_service = Arc::new(JwtService::new(&config.jwt_secret));

    server::run_http_server(config, pool, jwt_service).await?;

    Ok(())
}
