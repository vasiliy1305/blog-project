mod application;
mod data;
mod domain;
mod infrastructure;
mod presentation;

use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger, web};
use infrastructure::config::Config;
use sqlx::postgres::PgPoolOptions;
use tonic::Code::Ok;

use crate::infrastructure::database::{create_pool, run_migrations};

#[actix_web::main]
async fn main() {
    dotenvy::dotenv().ok();
    env_logger::init();

    let cfg = Config::from_env().expect("invalid config");
    let pool = create_pool(&cfg.database_url)
        .await
        .expect("failed to connect to database");
    run_migrations(&pool).await.expect("migrations failed");

    println!("{:?}", cfg);

    // let cors = Cors::default()
    //     .allowed_origin(&cfg.cors_origin)
    //     .allowed_methods(vec!["GET","POST","OPTIONS"])
    //     .allowed_headers(vec![
    //         actix_web::http::header::CONTENT_TYPE,
    //         actix_web::http::header::AUTHORIZATION,
    //     ])
    //     .supports_credentials()
    //     .max_age(600);

    // let addr = format!("{}:{}", cfg.host, cfg.port);
    // println!("→ listening on http://{}", addr);

    // HttpServer::new(move || {
    //     App::new()
    //         .wrap(Logger::default())
    //         .wrap(cors.clone())
    //         .app_data(web::Data::new(pool.clone()))
    //         .app_data(web::Data::new(cfg.clone()))
    //         .configure(presentation::routes::configure)
    // })
    // .bind(addr)?
    // .run()
    // .await
}
