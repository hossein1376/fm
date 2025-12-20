use actix_web::web;

mod auth;
mod hosts;
mod files;

pub use auth::*;
pub use hosts::*;
pub use files::*;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
    )
    .service(
        web::scope("/hosts")
            .route("", web::post().to(create_host))
            .route("", web::get().to(list_hosts))
            .route("/{id}", web::get().to(get_host))
            .route("/{id}", web::delete().to(delete_host))
    )
    .service(
        web::scope("/files")
            .route("/browse", web::post().to(browse_files))
            .route("/download", web::post().to(download_file))
            .route("/upload", web::post().to(upload_file))
            .route("/delete", web::post().to(delete_file))
            .route("/mkdir", web::post().to(create_directory))
    );
}
