use crate::auth::{create_jwt, hash_password, verify_password};
use crate::db::Database;
use crate::models::{AuthResponse, LoginRequest, RegisterRequest, User};
use actix_web::{web, HttpResponse};
use serde_json::json;
use std::sync::Arc;

pub async fn register(
    db: web::Data<Arc<Database>>,
    req: web::Json<RegisterRequest>,
) -> HttpResponse {
    // Check if user already exists
    match db.get_user_by_username(&req.username).await {
        Ok(Some(_)) => {
            return HttpResponse::Conflict().json(json!({
                "error": "Username already exists"
            }));
        }
        Ok(None) => {}
        Err(e) => {
            return HttpResponse::InternalServerError().json(json!({
                "error": format!("Database error: {}", e)
            }));
        }
    }

    // Hash password
    let password_hash = match hash_password(&req.password) {
        Ok(hash) => hash,
        Err(e) => {
            return HttpResponse::InternalServerError().json(json!({
                "error": format!("Failed to hash password: {}", e)
            }));
        }
    };

    // Create user
    let user = User::new(req.username.clone(), password_hash);

    match db.create_user(&user).await {
        Ok(created_user) => {
            match create_jwt(&created_user.id.to_string(), &created_user.username) {
                Ok(token) => HttpResponse::Ok().json(AuthResponse {
                    token,
                    user: created_user.to_user_info(),
                }),
                Err(e) => HttpResponse::InternalServerError().json(json!({
                    "error": format!("Failed to create token: {}", e)
                })),
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to create user: {}", e)
        })),
    }
}

pub async fn login(db: web::Data<Arc<Database>>, req: web::Json<LoginRequest>) -> HttpResponse {
    // Get user
    let user = match db.get_user_by_username(&req.username).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return HttpResponse::Unauthorized().json(json!({
                "error": "Invalid credentials"
            }));
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(json!({
                "error": format!("Database error: {}", e)
            }));
        }
    };

    // Verify password
    match verify_password(&req.password, &user.password_hash) {
        Ok(true) => match create_jwt(&user.id.to_string(), &user.username) {
            Ok(token) => HttpResponse::Ok().json(AuthResponse {
                token,
                user: user.to_user_info(),
            }),
            Err(e) => HttpResponse::InternalServerError().json(json!({
                "error": format!("Failed to create token: {}", e)
            })),
        },
        Ok(false) => HttpResponse::Unauthorized().json(json!({
            "error": "Invalid credentials"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to verify password: {}", e)
        })),
    }
}
