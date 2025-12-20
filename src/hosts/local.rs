use crate::models::FileInfo;
use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDateTime, Utc};
use std::path::{Path, PathBuf};
use tokio::fs;

pub struct LocalFileSystem;

impl LocalFileSystem {
    pub async fn list_files(base_path: &str, path: &str) -> Result<Vec<FileInfo>> {
        let full_path = Self::resolve_path(base_path, path)?;

        let mut entries = fs::read_dir(&full_path)
            .await
            .context("Failed to read directory")?;

        let mut files = Vec::new();

        while let Some(entry) = entries.next_entry().await? {
            let metadata = entry.metadata().await?;
            let file_name = entry.file_name().to_string_lossy().to_string();

            let modified = metadata.modified().ok().and_then(|time| {
                let duration = time.duration_since(std::time::UNIX_EPOCH).ok()?;
                let naive = NaiveDateTime::from_timestamp(
                    duration.as_secs() as i64,
                    duration.subsec_nanos(),
                );
                Some(DateTime::<Utc>::from_utc(naive, Utc))
            });

            let file_path = if path.is_empty() || path == "/" {
                format!("/{}", file_name)
            } else {
                format!("{}/{}", path.trim_end_matches('/'), file_name)
            };

            files.push(FileInfo {
                name: file_name,
                path: file_path,
                is_dir: metadata.is_dir(),
                size: metadata.len(),
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

    pub async fn read_file(base_path: &str, path: &str) -> Result<Vec<u8>> {
        let full_path = Self::resolve_path(base_path, path)?;
        let content = fs::read(&full_path).await?;
        Ok(content)
    }

    pub async fn write_file(base_path: &str, path: &str, content: &[u8]) -> Result<()> {
        let full_path = Self::resolve_path(base_path, path)?;

        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        fs::write(&full_path, content).await?;
        Ok(())
    }

    pub async fn delete_file(base_path: &str, path: &str) -> Result<()> {
        let full_path = Self::resolve_path(base_path, path)?;
        let metadata = fs::metadata(&full_path).await?;

        if metadata.is_dir() {
            fs::remove_dir_all(&full_path).await?;
        } else {
            fs::remove_file(&full_path).await?;
        }

        Ok(())
    }

    pub async fn create_directory(base_path: &str, path: &str) -> Result<()> {
        let full_path = Self::resolve_path(base_path, path)?;
        fs::create_dir_all(&full_path).await?;
        Ok(())
    }

    fn resolve_path(base_path: &str, path: &str) -> Result<PathBuf> {
        let base = Path::new(base_path)
            .canonicalize()
            .context("Invalid base path")?;

        let requested = base.join(path.trim_start_matches('/'));
        let resolved = if requested.exists() {
            requested.canonicalize()?
        } else {
            requested
        };

        // Ensure the resolved path is within the base path
        if !resolved.starts_with(&base) {
            anyhow::bail!("Path traversal attempt detected");
        }

        Ok(resolved)
    }
}
