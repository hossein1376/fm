use chrono::{DateTime, Utc};
use serde::de;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Host {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub host_type: HostType,
    pub config: HostConfig,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum HostType {
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "sftp")]
    Sftp,
}

fn deserialize_option_u16_from_string_or_number<'de, D>(
    deserializer: D,
) -> Result<Option<u16>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    // Deserialize into an Option<serde_json::Value> so we can accept either a number or a string.
    let opt = Option::<Value>::deserialize(deserializer)?;
    match opt {
        None => Ok(None),
        Some(Value::Number(n)) => {
            if let Some(u) = n.as_u64() {
                if u <= u16::MAX as u64 {
                    Ok(Some(u as u16))
                } else {
                    Err(de::Error::custom("port out of range"))
                }
            } else {
                Err(de::Error::custom("invalid number for port"))
            }
        }
        Some(Value::String(s)) => {
            let parsed = s.parse::<u16>().map_err(de::Error::custom)?;
            Ok(Some(parsed))
        }
        Some(_) => Err(de::Error::custom("invalid type for port")),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_u16_from_string_or_number"
    )]
    pub port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_encrypted: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: u64,
    pub modified: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateHostRequest {
    pub name: String,
    pub host_type: HostType,
    pub config: HostConfig,
}

#[derive(Debug, Deserialize)]
pub struct BrowseRequest {
    pub host_id: serde_json::Value,
    pub path: String,
}

#[derive(Debug, Serialize)]
pub struct BrowseResponse {
    pub path: String,
    pub files: Vec<FileInfo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WsMessage {
    pub event: String,
    pub data: serde_json::Value,
}

impl User {
    pub fn new(username: String, password_hash: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            username,
            password_hash,
            created_at: Utc::now(),
        }
    }

    pub fn to_user_info(&self) -> UserInfo {
        UserInfo {
            id: self.id.clone(),
            username: self.username.clone(),
            created_at: self.created_at.clone(),
        }
    }
}

impl Host {
    pub fn new(user_id: String, name: String, host_type: HostType, config: HostConfig) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            name,
            host_type,
            config,
            created_at: Utc::now(),
        }
    }
}
