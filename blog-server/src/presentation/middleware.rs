use actix_web::{
    Error, HttpMessage, dev::ServiceRequest, error::ErrorInternalServerError,
    error::ErrorUnauthorized, web,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::infrastructure::jwt::JwtService;

#[derive(Clone)]
pub struct AuthenticatedUser {
    pub user_id: i64,
    pub username: String,
}

pub async fn jwt_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let jwt_service = req.app_data::<web::Data<JwtService>>().cloned();

    match jwt_service {
        None => Err((
            ErrorInternalServerError("JwtService is not configured"),
            req,
        )),

        Some(jwt_service) => {
            let token = credentials.token();
            let claims = jwt_service.verify_token(token);

            match claims {
                Err(_) => Err((ErrorUnauthorized("Unauthorized"), req)),

                Ok(claims) => {
                    let user = AuthenticatedUser {
                        user_id: claims.user_id,
                        username: claims.username,
                    };
                    req.extensions_mut().insert(user);
                    Ok(req)
                }
            }
        }
    }
}
