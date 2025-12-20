use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web::error::ErrorUnauthorized;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use crate::auth::jwt::verify_jwt;

pub struct AuthMiddleware;

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    match verify_jwt(credentials.token()) {
        Ok(claims) => {
            req.extensions_mut().insert(claims);
            Ok(req)
        }
        Err(_) => Err((ErrorUnauthorized("Invalid token"), req)),
    }
}
