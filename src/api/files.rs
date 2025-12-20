use crate::auth::{verify_jwt, Encryptor};
use crate::db::Database;
use crate::hosts;
use crate::metrics::Metrics;
use crate::models::BrowseRequest;
use actix_multipart::Multipart;
use actix_web::{web, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use futures::StreamExt;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct FileActionRequest {
    pub host_id: serde_json::Value,
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct UploadRequest {
    pub host_id: String,
    pub path: String,
}

pub async fn browse_files(
    db: web::Data<Arc<Database>>,
    auth: BearerAuth,
    req: web::Json<BrowseRequest>,
) -> HttpResponse {
    let claims = match verify_jwt(auth.token()) {
        Ok(claims) => claims,
        Err(_) => {
            return HttpResponse::Unauthorized().json(json!({
                "error": "Invalid token"
            }));
        }
    };

    // Extract host_id string from Value
    let host_id_str = match &req.host_id {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Object(obj) => {
            if let Some(serde_json::Value::Object(id_obj)) = obj.get("id") {
                if let Some(serde_json::Value::String(s)) = id_obj.get("String") {
                    s.clone()
                } else {
                    return HttpResponse::BadRequest().json(json!({
                        "error": "Invalid host_id format"
                    }));
                }
            } else {
                return HttpResponse::BadRequest().json(json!({
                    "error": "Invalid host_id format"
                }));
            }
        }
        _ => {
            return HttpResponse::BadRequest().json(json!({
                "error": "Invalid host_id format"
            }));
        }
    };

    // Get host
    let host = match db.get_host(&host_id_str).await {
        Ok(Some(host)) => host,
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
    };

    // Verify ownership
    if host.user_id != claims.sub {
        return HttpResponse::Forbidden().json(json!({
            "error": "Access denied"
        }));
    }

    // Browse files
    let encryptor = match Encryptor::new() {
        Ok(enc) => enc,
        Err(e) => {
            return HttpResponse::InternalServerError().json(json!({
                "error": format!("Failed to initialize encryptor: {}", e)
            }));
        }
    };

    match hosts::browse_host(&host, &req.path, &encryptor).await {
        Ok(files) => HttpResponse::Ok().json(json!({
            "path": req.path,
            "files": files
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to browse files: {}", e)
        })),
    }
}

pub async fn download_file(
    db: web::Data<Arc<Database>>,
    metrics: web::Data<Arc<Metrics>>,
    auth: BearerAuth,
    req: web::Json<FileActionRequest>,
) -> HttpResponse {
    let claims = match verify_jwt(auth.token()) {
        Ok(claims) => claims,
        Err(_) => {
            return HttpResponse::Unauthorized().json(json!({
                "error": "Invalid token"
            }));
        }
    };

    // Extract host_id string from Value
    let host_id_str = match &req.host_id {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Object(obj) => {
            if let Some(serde_json::Value::Object(id_obj)) = obj.get("id") {
                if let Some(serde_json::Value::String(s)) = id_obj.get("String") {
                    s.clone()
                } else {
                    return HttpResponse::BadRequest().json(json!({
                        "error": "Invalid host_id format"
                    }));
                }
            } else {
                return HttpResponse::BadRequest().json(json!({
                    "error": "Invalid host_id format"
                }));
            }
        }
        _ => {
            return HttpResponse::BadRequest().json(json!({
                "error": "Invalid host_id format"
            }));
        }
    };

    // Get host
    let host = match db.get_host(&host_id_str).await {
        Ok(Some(host)) => host,
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
    };

    // Verify ownership
    if host.user_id != claims.sub {
        return HttpResponse::Forbidden().json(json!({
            "error": "Access denied"
        }));
    }

    let encryptor = match Encryptor::new() {
        Ok(enc) => enc,
        Err(e) => {
            return HttpResponse::InternalServerError().json(json!({
                "error": format!("Failed to initialize encryptor: {}", e)
            }));
        }
    };

    match hosts::read_file(&host, &req.path, &encryptor).await {
        Ok(content) => {
            metrics.file_downloads.inc();

            let mime_type = mime_guess::from_path(&req.path).first_or_octet_stream();
            HttpResponse::Ok()
                .content_type(mime_type.as_ref())
                .body(content)
        }
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to read file: {}", e)
        })),
    }
}

pub async fn upload_file(
    db: web::Data<Arc<Database>>,
    metrics: web::Data<Arc<Metrics>>,
    auth: BearerAuth,
    mut payload: Multipart,
) -> HttpResponse {
    let claims = match verify_jwt(auth.token()) {
        Ok(claims) => claims,
        Err(_) => {
            return HttpResponse::Unauthorized().json(json!({
                "error": "Invalid token"
            }));
        }
    };

    let mut host_id: Option<String> = None;
    let mut path: Option<String> = None;
    let mut file_data: Vec<u8> = Vec::new();

    // Parse multipart form data
    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(field) => field,
            Err(e) => {
                return HttpResponse::BadRequest().json(json!({
                    "error": format!("Failed to read field: {}", e)
                }));
            }
        };

        let content_disposition = field.content_disposition();
        let field_name = content_disposition.get_name().unwrap_or("");

        match field_name {
            "host_id" => {
                let mut bytes = Vec::new();
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    bytes.extend_from_slice(&data);
                }
                host_id = Some(String::from_utf8(bytes).unwrap_or_default());
            }
            "path" => {
                let mut bytes = Vec::new();
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    bytes.extend_from_slice(&data);
                }
                path = Some(String::from_utf8(bytes).unwrap_or_default());
            }
            "file" => {
                while let Some(chunk) = field.next().await {
                    let data = match chunk {
                        Ok(data) => data,
                        Err(e) => {
                            return HttpResponse::BadRequest().json(json!({
                                "error": format!("Failed to read chunk: {}", e)
                            }));
                        }
                    };
                    file_data.extend_from_slice(&data);
                }
            }
            _ => {}
        }
    }

    let host_id = match host_id {
        Some(id) => id,
        None => {
            return HttpResponse::BadRequest().json(json!({
                "error": "Missing host_id"
            }));
        }
    };

    let path = match path {
        Some(p) => p,
        None => {
            return HttpResponse::BadRequest().json(json!({
                "error": "Missing path"
            }));
        }
    };

    // Get host
    let host = match db.get_host(&host_id).await {
        Ok(Some(host)) => host,
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
    };

    // Verify ownership
    if host.user_id != claims.sub {
        return HttpResponse::Forbidden().json(json!({
            "error": "Access denied"
        }));
    }

    let encryptor = match Encryptor::new() {
        Ok(enc) => enc,
        Err(e) => {
            return HttpResponse::InternalServerError().json(json!({
                "error": format!("Failed to initialize encryptor: {}", e)
            }));
        }
    };

    match hosts::write_file(&host, &path, &file_data, &encryptor).await {
        Ok(_) => {
            metrics.file_uploads.inc();
            HttpResponse::Ok().json(json!({
                "message": "File uploaded successfully"
            }))
        }
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to upload file: {}", e)
        })),
    }
}

pub async fn delete_file(
    db: web::Data<Arc<Database>>,
    auth: BearerAuth,
    req: web::Json<FileActionRequest>,
) -> HttpResponse {
    let claims = match verify_jwt(auth.token()) {
        Ok(claims) => claims,
        Err(_) => {
            return HttpResponse::Unauthorized().json(json!({
                "error": "Invalid token"
            }));
        }
    };

    // Extract host_id string from Value
    let host_id_str = match &req.host_id {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Object(obj) => {
            if let Some(serde_json::Value::Object(id_obj)) = obj.get("id") {
                if let Some(serde_json::Value::String(s)) = id_obj.get("String") {
                    s.clone()
                } else {
                    return HttpResponse::BadRequest().json(json!({
                        "error": "Invalid host_id format"
                    }));
                }
            } else {
                return HttpResponse::BadRequest().json(json!({
                    "error": "Invalid host_id format"
                }));
            }
        }
        _ => {
            return HttpResponse::BadRequest().json(json!({
                "error": "Invalid host_id format"
            }));
        }
    };

    let host = match db.get_host(&host_id_str).await {
        Ok(Some(host)) => host,
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
    };

    if host.user_id != claims.sub {
        return HttpResponse::Forbidden().json(json!({
            "error": "Access denied"
        }));
    }

    // Only support local filesystem for delete
    match &host.host_type {
        crate::models::HostType::Local => {
            let base_path = host
                .config
                .path
                .as_ref()
                .ok_or_else(|| "Local path not configured");

            if let Err(e) = base_path {
                return HttpResponse::InternalServerError().json(json!({
                    "error": e
                }));
            }

            match crate::hosts::local::LocalFileSystem::delete_file(base_path.unwrap(), &req.path)
                .await
            {
                Ok(_) => HttpResponse::Ok().json(json!({
                    "message": "File deleted successfully"
                })),
                Err(e) => HttpResponse::InternalServerError().json(json!({
                    "error": format!("Failed to delete file: {}", e)
                })),
            }
        }
        _ => HttpResponse::BadRequest().json(json!({
            "error": "Delete operation only supported for local filesystem"
        })),
    }
}

pub async fn create_directory(
    db: web::Data<Arc<Database>>,
    auth: BearerAuth,
    req: web::Json<FileActionRequest>,
) -> HttpResponse {
    let claims = match verify_jwt(auth.token()) {
        Ok(claims) => claims,
        Err(_) => {
            return HttpResponse::Unauthorized().json(json!({
                "error": "Invalid token"
            }));
        }
    };

    // Extract host_id string from Value
    let host_id_str = match &req.host_id {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Object(obj) => {
            if let Some(serde_json::Value::Object(id_obj)) = obj.get("id") {
                if let Some(serde_json::Value::String(s)) = id_obj.get("String") {
                    s.clone()
                } else {
                    return HttpResponse::BadRequest().json(json!({
                        "error": "Invalid host_id format"
                    }));
                }
            } else {
                return HttpResponse::BadRequest().json(json!({
                    "error": "Invalid host_id format"
                }));
            }
        }
        _ => {
            return HttpResponse::BadRequest().json(json!({
                "error": "Invalid host_id format"
            }));
        }
    };

    let host = match db.get_host(&host_id_str).await {
        Ok(Some(host)) => host,
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
    };

    if host.user_id != claims.sub {
        return HttpResponse::Forbidden().json(json!({
            "error": "Access denied"
        }));
    }

    // Only support local filesystem for mkdir
    match &host.host_type {
        crate::models::HostType::Local => {
            let base_path = host
                .config
                .path
                .as_ref()
                .ok_or_else(|| "Local path not configured");

            if let Err(e) = base_path {
                return HttpResponse::InternalServerError().json(json!({
                    "error": e
                }));
            }

            match crate::hosts::local::LocalFileSystem::create_directory(
                base_path.unwrap(),
                &req.path,
            )
            .await
            {
                Ok(_) => HttpResponse::Ok().json(json!({
                    "message": "Directory created successfully"
                })),
                Err(e) => HttpResponse::InternalServerError().json(json!({
                    "error": format!("Failed to create directory: {}", e)
                })),
            }
        }
        _ => HttpResponse::BadRequest().json(json!({
            "error": "Create directory operation only supported for local filesystem"
        })),
    }
}
