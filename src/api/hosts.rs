use crate::auth::{verify_jwt, Encryptor};
use crate::db::Database;
use crate::models::{CreateHostRequest, Host};
use actix_web::{web, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde_json::json;
use std::sync::Arc;

pub async fn create_host(
    db: web::Data<Arc<Database>>,
    auth: BearerAuth,
    req: web::Json<CreateHostRequest>,
) -> HttpResponse {
    // Verify JWT
    let claims = match verify_jwt(auth.token()) {
        Ok(claims) => claims,
        Err(_) => {
            return HttpResponse::Unauthorized().json(json!({
                "error": "Invalid token"
            }));
        }
    };

    // Encrypt password if present
    let mut config = req.config.clone();
    if let Some(password) = &config.password_encrypted {
        let encryptor = match Encryptor::new() {
            Ok(enc) => enc,
            Err(e) => {
                return HttpResponse::InternalServerError().json(json!({
                    "error": format!("Failed to initialize encryptor: {}", e)
                }));
            }
        };

        match encryptor.encrypt(password) {
            Ok(encrypted) => {
                config.password_encrypted = Some(encrypted);
            }
            Err(e) => {
                return HttpResponse::InternalServerError().json(json!({
                    "error": format!("Failed to encrypt password: {}", e)
                }));
            }
        }
    }

    let host = Host::new(
        claims.sub.clone(),
        req.name.clone(),
        req.host_type.clone(),
        config,
    );

    match db.create_host(&host).await {
        Ok(created_host) => {
            // Log success for auditing/debugging
            log::info!(
                "Host created (id: {}) for user {}",
                created_host.id,
                claims.sub
            );
            HttpResponse::Ok().json(created_host)
        }
        Err(e) => {
            // Log the detailed error (including debug info)
            log::error!("Failed to create host for user {}: {:#}", claims.sub, e);

            // Provide clearer client-visible errors for known cases
            let err_string = format!("{}", e);
            if err_string.contains("User not found") {
                return HttpResponse::BadRequest().json(json!({
                    "error": "User not found",
                    "details": err_string
                }));
            }

            // Return DB error details in response to aid debugging (caller requested)
            // Note: this may expose DB messages; ensure this is acceptable in your environment.
            return HttpResponse::InternalServerError().json(json!({
                "error": "Failed to create host",
                "details": err_string
            }));
        }
    }
}

pub async fn list_hosts(db: web::Data<Arc<Database>>, auth: BearerAuth) -> HttpResponse {
    let claims = match verify_jwt(auth.token()) {
        Ok(claims) => claims,
        Err(_) => {
            return HttpResponse::Unauthorized().json(json!({
                "error": "Invalid token"
            }));
        }
    };

    match db.get_hosts_by_user(&claims.sub).await {
        Ok(hosts) => HttpResponse::Ok().json(hosts),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to get hosts: {}", e)
        })),
    }
}

pub async fn get_host(
    db: web::Data<Arc<Database>>,
    auth: BearerAuth,
    path: web::Path<String>,
) -> HttpResponse {
    let claims = match verify_jwt(auth.token()) {
        Ok(claims) => claims,
        Err(_) => {
            return HttpResponse::Unauthorized().json(json!({
                "error": "Invalid token"
            }));
        }
    };

    let host_id = path.into_inner();

    match db.get_host(&host_id).await {
        Ok(Some(host)) => {
            if host.user_id != claims.sub {
                return HttpResponse::Forbidden().json(json!({
                    "error": "Access denied"
                }));
            }
            HttpResponse::Ok().json(host)
        }
        Ok(None) => HttpResponse::NotFound().json(json!({
            "error": "Host not found"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to get host: {}", e)
        })),
    }
}

pub async fn delete_host(
    db: web::Data<Arc<Database>>,
    auth: BearerAuth,
    path: web::Path<String>,
) -> HttpResponse {
    let claims = match verify_jwt(auth.token()) {
        Ok(claims) => claims,
        Err(_) => {
            return HttpResponse::Unauthorized().json(json!({
                "error": "Invalid token"
            }));
        }
    };

    let host_id = path.into_inner();

    // Verify ownership
    match db.get_host(&host_id).await {
        Ok(Some(host)) => {
            if host.user_id != claims.sub {
                return HttpResponse::Forbidden().json(json!({
                    "error": "Access denied"
                }));
            }
        }
        Ok(None) => {
            return HttpResponse::NotFound().json(json!({
                "error": "Host not found"
            }));
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(json!({
                "error": format!("Failed to get host: {}", e)
            }));
        }
    }

    match db.delete_host(&host_id).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "message": "Host deleted successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to delete host: {}", e)
        })),
    }
}
