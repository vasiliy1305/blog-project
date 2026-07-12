use actix_web::{
    HttpMessage, HttpRequest, HttpResponse, Responder, delete, error::ErrorUnauthorized, get, post,
    put, web,
};
use serde::Deserialize;

use crate::application::{auth_service::AuthService, blog_service::BlogService};
use crate::data::{
    post_repository::PostgresPostRepository, user_repository::PostgresUserRepository,
};
use crate::domain::{
    post::{CreatePost, UpdatePost},
    user::{Login, Registration},
};
use crate::infrastructure::password::PasswordArgon2;
use crate::presentation::middleware::AuthenticatedUser;

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
}

#[get("/api/posts/{id}")]
pub async fn get_post(
    service: web::Data<BlogService<PostgresPostRepository>>,
    id: web::Path<i64>,
) -> actix_web::Result<impl Responder> {
    let id = id.into_inner();
    let post = service.get(id).await?;
    Ok(HttpResponse::Ok().json(post))
}

#[derive(Deserialize)]
struct ListQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

#[get("/api/posts")]
pub async fn get_posts(
    service: web::Data<BlogService<PostgresPostRepository>>,
    query: web::Query<ListQuery>,
) -> actix_web::Result<impl Responder> {
    let request = query.into_inner();
    let limit = request.limit.unwrap_or(10);
    let offset = request.offset.unwrap_or(0);

    let posts = service.list(limit, offset).await?;
    Ok(HttpResponse::Ok().json(posts))
}

#[post("/api/posts")]
pub async fn create_post(
    req: HttpRequest,
    service: web::Data<BlogService<PostgresPostRepository>>,
    body: web::Json<CreatePost>,
) -> actix_web::Result<impl Responder> {
    let user = req
        .extensions()
        .get::<AuthenticatedUser>()
        .cloned()
        .ok_or_else(|| ErrorUnauthorized("Unauthorized"))?;

    let create_data = body.into_inner();
    let post = service.create(&create_data, user.user_id).await?;
    Ok(HttpResponse::Created().json(post))
}

#[put("/api/posts/{id}")]
pub async fn update_post(
    req: HttpRequest,
    service: web::Data<BlogService<PostgresPostRepository>>,
    id: web::Path<i64>,
    body: web::Json<UpdatePost>,
) -> actix_web::Result<impl Responder> {
    let user = req
        .extensions()
        .get::<AuthenticatedUser>()
        .cloned()
        .ok_or_else(|| ErrorUnauthorized("Unauthorized"))?;

    let post_id = id.into_inner();
    let update_data = body.into_inner();

    let post = service.update(post_id, user.user_id, &update_data).await?;

    Ok(HttpResponse::Ok().json(post))
}

#[delete("/api/posts/{id}")]
pub async fn delete_post(
    req: HttpRequest,
    service: web::Data<BlogService<PostgresPostRepository>>,
    id: web::Path<i64>,
) -> actix_web::Result<impl Responder> {
    let user = req
        .extensions()
        .get::<AuthenticatedUser>()
        .cloned()
        .ok_or_else(|| ErrorUnauthorized("Unauthorized"))?;

    let post_id = id.into_inner();

    service.delete(post_id, user.user_id).await?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/api/auth/register")]
pub async fn register(
    service: web::Data<AuthService<PostgresUserRepository, PasswordArgon2>>,
    body: web::Json<Registration>,
) -> actix_web::Result<impl Responder> {
    let registration = body.into_inner();

    let response = service.register(&registration).await?;

    Ok(HttpResponse::Created().json(response))
}

#[post("/api/auth/login")]
pub async fn login(
    service: web::Data<AuthService<PostgresUserRepository, PasswordArgon2>>,
    body: web::Json<Login>,
) -> actix_web::Result<impl Responder> {
    let login_info = body.into_inner();

    let response = service.login(login_info).await?;

    Ok(HttpResponse::Ok().json(response))
}
