pub mod local;
pub mod http;
pub mod sftp;

use anyhow::Result;
use crate::models::{Host, HostType, FileInfo};
use crate::auth::Encryptor;

pub async fn browse_host(host: &Host, path: &str, encryptor: &Encryptor) -> Result<Vec<FileInfo>> {
    match &host.host_type {
        HostType::Local => {
            let base_path = host.config.path.as_ref()
                .ok_or_else(|| anyhow::anyhow!("Local path not configured"))?;
            local::LocalFileSystem::list_files(base_path, path).await
        }
        HostType::Http => {
            let base_url = host.config.url.as_ref()
                .ok_or_else(|| anyhow::anyhow!("HTTP URL not configured"))?;
            let http_fs = http::HttpFileSystem::new();
            http_fs.list_files(base_url, path).await
        }
        HostType::Sftp => {
            let host_addr = host.config.host.as_ref()
                .ok_or_else(|| anyhow::anyhow!("SFTP host not configured"))?;
            let port = host.config.port.unwrap_or(22);
            let username = host.config.username.as_ref()
                .ok_or_else(|| anyhow::anyhow!("SFTP username not configured"))?;
            let password_encrypted = host.config.password_encrypted.as_ref()
                .ok_or_else(|| anyhow::anyhow!("SFTP password not configured"))?;
            
            let password = encryptor.decrypt(password_encrypted)?;
            
            sftp::SftpFileSystem::list_files(
                host_addr,
                port,
                username,
                &password,
                path,
            ).await
        }
    }
}

pub async fn read_file(host: &Host, path: &str, encryptor: &Encryptor) -> Result<Vec<u8>> {
    match &host.host_type {
        HostType::Local => {
            let base_path = host.config.path.as_ref()
                .ok_or_else(|| anyhow::anyhow!("Local path not configured"))?;
            local::LocalFileSystem::read_file(base_path, path).await
        }
        HostType::Http => {
            let base_url = host.config.url.as_ref()
                .ok_or_else(|| anyhow::anyhow!("HTTP URL not configured"))?;
            let http_fs = http::HttpFileSystem::new();
            http_fs.read_file(base_url, path).await
        }
        HostType::Sftp => {
            let host_addr = host.config.host.as_ref()
                .ok_or_else(|| anyhow::anyhow!("SFTP host not configured"))?;
            let port = host.config.port.unwrap_or(22);
            let username = host.config.username.as_ref()
                .ok_or_else(|| anyhow::anyhow!("SFTP username not configured"))?;
            let password_encrypted = host.config.password_encrypted.as_ref()
                .ok_or_else(|| anyhow::anyhow!("SFTP password not configured"))?;
            
            let password = encryptor.decrypt(password_encrypted)?;
            
            sftp::SftpFileSystem::read_file(
                host_addr,
                port,
                username,
                &password,
                path,
            ).await
        }
    }
}

pub async fn write_file(host: &Host, path: &str, content: &[u8], encryptor: &Encryptor) -> Result<()> {
    match &host.host_type {
        HostType::Local => {
            let base_path = host.config.path.as_ref()
                .ok_or_else(|| anyhow::anyhow!("Local path not configured"))?;
            local::LocalFileSystem::write_file(base_path, path, content).await
        }
        HostType::Http => {
            anyhow::bail!("Write operation not supported for HTTP hosts")
        }
        HostType::Sftp => {
            let host_addr = host.config.host.as_ref()
                .ok_or_else(|| anyhow::anyhow!("SFTP host not configured"))?;
            let port = host.config.port.unwrap_or(22);
            let username = host.config.username.as_ref()
                .ok_or_else(|| anyhow::anyhow!("SFTP username not configured"))?;
            let password_encrypted = host.config.password_encrypted.as_ref()
                .ok_or_else(|| anyhow::anyhow!("SFTP password not configured"))?;
            
            let password = encryptor.decrypt(password_encrypted)?;
            
            sftp::SftpFileSystem::write_file(
                host_addr,
                port,
                username,
                &password,
                path,
                content,
            ).await
        }
    }
}
