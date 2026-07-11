use actix_web::{
    Error,
    HttpMessage,
    dev::ServiceRequest,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::infrastructure::jwt::JwtService;

pub struct AuthenticatedUser {
    pub user_id: i64,
    pub username: String,
}

pub async fn jwt_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let jwt_service = req
    .app_data::<actix_web::web::Data<JwtService>>();


    
    todo!()
}