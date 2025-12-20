use crate::models::{Host, HostConfig, HostType, User};
use anyhow::{anyhow, Context, Result};
use log::info;
use serde_json;

use sqlx::{Row, SqlitePool};
use std::env;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    /// Create a new Database backed by sqlite.
    /// Uses DATABASE_URL env var or defaults to `sqlite://fm.db`.
    pub async fn new() -> Result<Self> {
        let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://fm.db".to_string());

        info!("Connecting to SQLite at {}", db_url);

        // Ensure parent directory and the DB file exist and are writable when using file-based sqlite URLs.
        // Handles URIs like: sqlite://./data/fm.db or sqlite:///absolute/path/fm.db
        if let Some(path_str) = db_url.strip_prefix("sqlite://") {
            // Skip in-memory usages like sqlite::memory: or explicit memory keywords
            if !path_str.is_empty() && !path_str.contains("memory") {
                // Build the db_path: if absolute use as-is, otherwise resolve against current working directory.
                // This avoids relying on the caller's cwd semantics and ensures files/directories
                // are created relative to the process working directory.
                let mut db_path = std::path::PathBuf::from(path_str);
                if !db_path.is_absolute() {
                    let cwd = std::env::current_dir()
                        .context("Failed to determine current working directory")?;
                    db_path = cwd.join(db_path);
                }

                // Create parent directories if needed (don't canonicalize since file may not exist)
                if let Some(parent) = db_path.parent() {
                    // Avoid trying to create the current directory (".") or empty components.
                    if !parent.as_os_str().is_empty() && !parent.exists() {
                        std::fs::create_dir_all(parent).with_context(|| {
                            format!(
                                "Failed to create parent directory for SQLite DB at {:?}",
                                parent
                            )
                        })?;
                    }
                }

                // Ensure the DB file exists; create it if missing
                if !db_path.exists() {
                    std::fs::OpenOptions::new()
                        .create(true)
                        .write(true)
                        .open(&db_path)
                        .with_context(|| {
                            format!("Failed to create SQLite DB file at {:?}", db_path)
                        })?;
                }

                // Verify writability by opening for append (this will fail if not writable)
                std::fs::OpenOptions::new()
                    .append(true)
                    .open(&db_path)
                    .with_context(|| format!("SQLite DB file is not writable: {:?}", db_path))?;
            }
        }

        let pool = SqlitePool::connect(&db_url)
            .await
            .context("Failed to connect to SQLite database")?;

        // Apply recommended SQLite PRAGMAs for safer defaults:
        // - enable foreign keys
        // - set a busy timeout to reduce transient SQLITE_BUSY failures
        // - enable WAL (Write-Ahead Logging) for better concurrency
        sqlx::query("PRAGMA foreign_keys = ON;")
            .execute(&pool)
            .await
            .context("Failed to set PRAGMA foreign_keys")?;
        sqlx::query("PRAGMA busy_timeout = 5000;")
            .execute(&pool)
            .await
            .context("Failed to set PRAGMA busy_timeout")?;
        sqlx::query("PRAGMA journal_mode = WAL;")
            .execute(&pool)
            .await
            .context("Failed to set PRAGMA journal_mode")?;

        info!("Connected to SQLite");

        Ok(Self { pool })
    }

    /// Initialize schema (idempotent).
    pub async fn initialize(&self) -> Result<()> {
        // users table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                username TEXT NOT NULL UNIQUE,
                password_hash TEXT NOT NULL,
                created_at TEXT NOT NULL
            );
        "#,
        )
        .execute(&self.pool)
        .await
        .context("Failed to create users table")?;

        // hosts table, config stored as JSON text, host_type as text
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS hosts (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                name TEXT NOT NULL,
                host_type TEXT NOT NULL,
                config TEXT NOT NULL,
                created_at TEXT NOT NULL,
                FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE
            );
        "#,
        )
        .execute(&self.pool)
        .await
        .context("Failed to create hosts table")?;

        Ok(())
    }

    pub async fn create_user(&self, user: &User) -> Result<User> {
        let created_at = user.created_at.to_rfc3339();
        sqlx::query(
            r#"
            INSERT INTO users (id, username, password_hash, created_at)
            VALUES (?, ?, ?, ?)
        "#,
        )
        .bind(&user.id)
        .bind(&user.username)
        .bind(&user.password_hash)
        .bind(&created_at)
        .execute(&self.pool)
        .await
        .context("Failed to insert user")?;

        Ok(user.clone())
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>> {
        let row = sqlx::query(
            "SELECT id, username, password_hash, created_at FROM users WHERE username = ?",
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .context("Failed to query user by username")?;

        if let Some(r) = row {
            let id: String = r.try_get("id")?;
            let username: String = r.try_get("username")?;
            let password_hash: String = r.try_get("password_hash")?;
            let created_at_str: String = r.try_get("created_at")?;
            let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .context("Failed to parse user created_at")?;

            Ok(Some(User {
                id,
                username,
                password_hash,
                created_at,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn create_host(&self, host: &Host) -> Result<Host> {
        // Verify the user exists before inserting a host to provide a clearer error.
        let user_exists = sqlx::query("SELECT 1 FROM users WHERE id = ?")
            .bind(&host.user_id)
            .fetch_optional(&self.pool)
            .await
            .context("Failed to verify user existence")?;

        if user_exists.is_none() {
            // Return a clear error for callers/handlers to surface as a 4xx/5xx response.
            return Err(anyhow!("User not found"));
        }

        let created_at = host.created_at.to_rfc3339();
        let host_type = match &host.host_type {
            HostType::Local => "local",
            HostType::Http => "http",
            HostType::Sftp => "sftp",
        }
        .to_string();

        let config_json =
            serde_json::to_string(&host.config).context("Failed to serialize host config")?;

        // Attempt insert and capture SQL error to log details while returning a safe message upstream.
        let insert_result = sqlx::query(
            r#"
            INSERT INTO hosts (id, user_id, name, host_type, config, created_at)
            VALUES (?, ?, ?, ?, ?, ?)
        "#,
        )
        .bind(&host.id)
        .bind(&host.user_id)
        .bind(&host.name)
        .bind(&host_type)
        .bind(&config_json)
        .bind(&created_at)
        .execute(&self.pool)
        .await;

        match insert_result {
            Ok(_) => Ok(host.clone()),
            Err(e) => {
                // Log detailed SQL error for debugging (do not leak secrets to client)
                log::error!(
                    "SQL insert failed for host (host_id={}, user_id={}): {:?}",
                    host.id,
                    host.user_id,
                    e
                );

                // Return an anyhow error with the SQL error message so handlers can decide response
                return Err(anyhow!("Failed to insert host: {}", e));
            }
        }
    }

    pub async fn get_host(&self, host_id: &str) -> Result<Option<Host>> {
        let row = sqlx::query(
            "SELECT id, user_id, name, host_type, config, created_at FROM hosts WHERE id = ?",
        )
        .bind(host_id)
        .fetch_optional(&self.pool)
        .await
        .context("Failed to query host by id")?;

        if let Some(r) = row {
            let id: String = r.try_get("id")?;
            let user_id: String = r.try_get("user_id")?;
            let name: String = r.try_get("name")?;
            let host_type_str: String = r.try_get("host_type")?;
            let config_str: String = r.try_get("config")?;
            let created_at_str: String = r.try_get("created_at")?;
            let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .context("Failed to parse host created_at")?;

            let host_type = match host_type_str.as_str() {
                "local" => HostType::Local,
                "http" => HostType::Http,
                "sftp" => HostType::Sftp,
                other => {
                    // default to local if unknown
                    log::warn!("Unknown host_type '{}' for host {}", other, id);
                    HostType::Local
                }
            };

            let config: HostConfig =
                serde_json::from_str(&config_str).context("Failed to deserialize host config")?;

            Ok(Some(Host {
                id,
                user_id,
                name,
                host_type,
                config,
                created_at,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn get_hosts_by_user(&self, user_id: &str) -> Result<Vec<Host>> {
        let rows = sqlx::query(
            "SELECT id, user_id, name, host_type, config, created_at FROM hosts WHERE user_id = ?",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .context("Failed to query hosts by user")?;

        let mut hosts = Vec::new();
        for r in rows {
            let id: String = r.try_get("id")?;
            let user_id: String = r.try_get("user_id")?;
            let name: String = r.try_get("name")?;
            let host_type_str: String = r.try_get("host_type")?;
            let config_str: String = r.try_get("config")?;
            let created_at_str: String = r.try_get("created_at")?;
            let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .context("Failed to parse host created_at")?;

            let host_type = match host_type_str.as_str() {
                "local" => HostType::Local,
                "http" => HostType::Http,
                "sftp" => HostType::Sftp,
                other => {
                    log::warn!("Unknown host_type '{}' for host {}", other, id);
                    HostType::Local
                }
            };

            let config: HostConfig =
                serde_json::from_str(&config_str).context("Failed to deserialize host config")?;

            hosts.push(Host {
                id,
                user_id,
                name,
                host_type,
                config,
                created_at,
            });
        }

        Ok(hosts)
    }

    pub async fn delete_host(&self, host_id: &str) -> Result<()> {
        sqlx::query("DELETE FROM hosts WHERE id = ?")
            .bind(host_id)
            .execute(&self.pool)
            .await
            .context("Failed to delete host")?;

        Ok(())
    }
}
