use crate::data::post_repository::PostgresPostRepository;
use actix_web::{HttpResponse, Responder, get, post, web};
use serde::{Deserialize, Serialize};


use crate::application::{auth_service, blog_service::BlogService};
use crate::domain::error::DomainError;


// POST /api/auth/register
// POST /api/auth/login
// POST /api/posts
// GET /api/posts/{id}
// GET /api/posts ?limit=10&offset=0
// PUT /api/posts/{id}
// DELETE /api/posts/{id}

// register
// login

// create_post
// get_post
// update_post
// delete_post
// list_posts

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
}

#[get("/api/posts/{id}")]
async fn get_post(
    service: web::Data<BlogService<PostgresPostRepository>>,
    id: web::Path<i64>,
) -> actix_web::Result<impl Responder> {
    let id = id.into_inner();
    let post = service.get(id).await?;
    Ok(HttpResponse::Ok().json(post))
}

#[derive(Deserialize)]
struct ListRequest {
    limit: Option<i64>,
    offset: Option<i64>,
}

#[get("/api/posts")]
async fn get_posts(
    service: web::Data<BlogService<PostgresPostRepository>>,
    query: web::Query<ListRequest>,
) -> actix_web::Result<impl Responder> {
    let request = query.into_inner();
    let limit = request.limit.unwrap_or(10);
    let offset = request.offset.unwrap_or(0);

    let posts = service.list(limit, offset).await?;
    Ok(HttpResponse::Ok().json(posts))
}
