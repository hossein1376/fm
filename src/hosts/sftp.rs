use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDateTime, Utc};
use ssh2::Session;
use std::net::TcpStream;
use std::path::Path;

use crate::models::FileInfo;

pub struct SftpFileSystem;

impl SftpFileSystem {
    pub async fn list_files(
        host: &str,
        port: u16,
        username: &str,
        password: &str,
        path: &str,
    ) -> Result<Vec<FileInfo>> {
        // Use spawn_blocking for synchronous SSH operations
        let host = host.to_string();
        let username = username.to_string();
        let password = password.to_string();
        let path = path.to_string();

        tokio::task::spawn_blocking(move || {
            Self::list_files_sync(&host, port, &username, &password, &path)
        })
        .await?
    }

    fn list_files_sync(
        host: &str,
        port: u16,
        username: &str,
        password: &str,
        path: &str,
    ) -> Result<Vec<FileInfo>> {
        let tcp = TcpStream::connect(format!("{}:{}", host, port))
            .context("Failed to connect to SFTP server")?;

        let mut sess = Session::new().context("Failed to create SSH session")?;
        sess.set_tcp_stream(tcp);
        sess.handshake().context("SSH handshake failed")?;
        sess.userauth_password(username, password)
            .context("SSH authentication failed")?;

        let sftp = sess.sftp().context("Failed to create SFTP session")?;

        let entries = sftp
            .readdir(Path::new(path))
            .context("Failed to read directory")?;

        let mut files = Vec::new();

        for (path_buf, stat) in entries {
            let file_name = path_buf
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();

            let file_path = path_buf.to_string_lossy().to_string();

            let modified = stat.mtime.and_then(|mtime| {
                let naive = NaiveDateTime::from_timestamp(mtime as i64, 0);
                Some(DateTime::<Utc>::from_utc(naive, Utc))
            });

            files.push(FileInfo {
                name: file_name,
                path: file_path,
                is_dir: stat.is_dir(),
                size: stat.size.unwrap_or(0),
                modified,
            });
        }

        files.sort_by(|a, b| match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.cmp(&b.name),
        });

        Ok(files)
    }

    pub async fn read_file(
        host: &str,
        port: u16,
        username: &str,
        password: &str,
        path: &str,
    ) -> Result<Vec<u8>> {
        let host = host.to_string();
        let username = username.to_string();
        let password = password.to_string();
        let path = path.to_string();

        tokio::task::spawn_blocking(move || {
            Self::read_file_sync(&host, port, &username, &password, &path)
        })
        .await?
    }

    fn read_file_sync(
        host: &str,
        port: u16,
        username: &str,
        password: &str,
        path: &str,
    ) -> Result<Vec<u8>> {
        let tcp = TcpStream::connect(format!("{}:{}", host, port))
            .context("Failed to connect to SFTP server")?;

        let mut sess = Session::new().context("Failed to create SSH session")?;
        sess.set_tcp_stream(tcp);
        sess.handshake().context("SSH handshake failed")?;
        sess.userauth_password(username, password)
            .context("SSH authentication failed")?;

        let sftp = sess.sftp().context("Failed to create SFTP session")?;

        let mut remote_file = sftp
            .open(Path::new(path))
            .context("Failed to open remote file")?;

        let mut buffer = Vec::new();
        std::io::Read::read_to_end(&mut remote_file, &mut buffer).context("Failed to read file")?;

        Ok(buffer)
    }

    pub async fn write_file(
        host: &str,
        port: u16,
        username: &str,
        password: &str,
        path: &str,
        content: &[u8],
    ) -> Result<()> {
        let host = host.to_string();
        let username = username.to_string();
        let password = password.to_string();
        let path = path.to_string();
        let content = content.to_vec();

        tokio::task::spawn_blocking(move || {
            Self::write_file_sync(&host, port, &username, &password, &path, &content)
        })
        .await?
    }

    fn write_file_sync(
        host: &str,
        port: u16,
        username: &str,
        password: &str,
        path: &str,
        content: &[u8],
    ) -> Result<()> {
        let tcp = TcpStream::connect(format!("{}:{}", host, port))
            .context("Failed to connect to SFTP server")?;

        let mut sess = Session::new().context("Failed to create SSH session")?;
        sess.set_tcp_stream(tcp);
        sess.handshake().context("SSH handshake failed")?;
        sess.userauth_password(username, password)
            .context("SSH authentication failed")?;

        let sftp = sess.sftp().context("Failed to create SFTP session")?;

        let mut remote_file = sftp
            .create(Path::new(path))
            .context("Failed to create remote file")?;

        std::io::Write::write_all(&mut remote_file, content).context("Failed to write file")?;

        Ok(())
    }
}
