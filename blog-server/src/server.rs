use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger, web};
use actix_web_httpauth::middleware::HttpAuthentication;
use sqlx::PgPool;

use crate::application::{auth_service::AuthService, blog_service::BlogService};
use crate::data::{
    post_repository::PostgresPostRepository, user_repository::PostgresUserRepository,
};
use crate::infrastructure::{config::Config, jwt::JwtService, password::PasswordArgon2};
use crate::presentation::{
    http_handlers::{
        create_post, delete_post, get_post, get_posts, health, login, register, update_post,
    },
    middleware::jwt_validator,
};

use std::sync::Arc;

pub async fn run_http_server(
    config: Config,
    pool: PgPool,
    jwt_service: Arc<JwtService>,
) -> std::io::Result<()> {
    let address = format!("{}:{}", config.host, config.port);
    let user_repository = PostgresUserRepository::new(pool.clone());
    let post_repository = PostgresPostRepository::new(pool);

    let auth_jwt_service = JwtService::new(&config.jwt_secret);
    let middleware_jwt_service = JwtService::new(&config.jwt_secret);

    let auth_service = web::Data::new(AuthService::new(
        jwt_service.clone(),
        user_repository,
        PasswordArgon2 {},
    ));

    let blog_service = web::Data::new(BlogService::new(post_repository));
    let jwt_service = web::Data::new(middleware_jwt_service);

    tracing::info!("HTTP server listening on http://{address}");
    HttpServer::new(move || {
        let authentication = HttpAuthentication::bearer(jwt_validator);

        let cors = if config.cors_origin == "*" {
            Cors::permissive()
        } else {
            Cors::default()
                .allowed_origin(&config.cors_origin)
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                .allow_any_header()
                .max_age(600)
        };

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(auth_service.clone())
            .app_data(blog_service.clone())
            .app_data(jwt_service.clone())
            .service(health)
            .service(register)
            .service(login)
            .service(get_posts)
            .service(get_post)
            .service(
                web::scope("")
                    .wrap(authentication)
                    .service(create_post)
                    .service(update_post)
                    .service(delete_post),
            )
    })
    .bind(&address)?
    .run()
    .await
}
