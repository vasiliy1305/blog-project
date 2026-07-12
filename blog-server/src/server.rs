use std::sync::Arc;

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

use crate::presentation::grpc_service::BlogGrpcService;
use blog_proto::blog::blog_service_server::BlogServiceServer;
use std::net::SocketAddr;
use tonic::transport::Server;

pub async fn run_http_server(
    config: Config,
    pool: PgPool,
    jwt_service: Arc<JwtService>,
) -> std::io::Result<()> {
    let address = format!("{}:{}", config.host, config.port);
    let grpc_address: SocketAddr = "0.0.0.0:50051".parse().expect("invalid grpc address");

    let user_repository = PostgresUserRepository::new(pool.clone());
    let post_repository = PostgresPostRepository::new(pool);

    let auth_service = Arc::new(AuthService::new(
        jwt_service.clone(),
        user_repository,
        PasswordArgon2 {},
    ));

    let blog_service = Arc::new(BlogService::new(post_repository));

    let grpc_service = BlogGrpcService::new(
        auth_service.clone(),
        blog_service.clone(),
        jwt_service.clone(),
    );

    tracing::info!("gRPC server listening on http://{grpc_address}");

    let grpc_server = async move {
        Server::builder()
            .add_service(BlogServiceServer::new(grpc_service))
            .serve(grpc_address)
            .await
            .map_err(std::io::Error::other)
    };

    let auth_service_data = web::Data::from(auth_service);
    let blog_service_data = web::Data::from(blog_service);
    let jwt_service_data = web::Data::from(jwt_service);

    tracing::info!("HTTP server listening on http://{address}");

    let http_server = HttpServer::new(move || {
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
            .app_data(auth_service_data.clone())
            .app_data(blog_service_data.clone())
            .app_data(jwt_service_data.clone())
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
    .run();

    tokio::try_join!(http_server, grpc_server,)?;

    Ok(())
}
