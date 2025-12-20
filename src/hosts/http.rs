use reqwest::Client;
use anyhow::{Result, Context};
use crate::models::FileInfo;

pub struct HttpFileSystem {
    client: Client,
}

impl HttpFileSystem {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn list_files(&self, base_url: &str, path: &str) -> Result<Vec<FileInfo>> {
        let url = format!("{}/{}", base_url.trim_end_matches('/'), path.trim_start_matches('/'));
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to send HTTP request")?;

        if !response.status().is_success() {
            anyhow::bail!("HTTP request failed with status: {}", response.status());
        }

        // Parse directory listing (this is a simplified implementation)
        // In a real scenario, you'd parse HTML or use a specific API
        let files = vec![]; // Placeholder
        Ok(files)
    }

    pub async fn read_file(&self, base_url: &str, path: &str) -> Result<Vec<u8>> {
        let url = format!("{}/{}", base_url.trim_end_matches('/'), path.trim_start_matches('/'));
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to send HTTP request")?;

        if !response.status().is_success() {
            anyhow::bail!("HTTP request failed with status: {}", response.status());
        }

        let bytes = response.bytes().await?.to_vec();
        Ok(bytes)
    }

    pub async fn download_file(&self, url: &str) -> Result<Vec<u8>> {
        let response = self.client
            .get(url)
            .send()
            .await
            .context("Failed to download file")?;

        if !response.status().is_success() {
            anyhow::bail!("Download failed with status: {}", response.status());
        }

        let bytes = response.bytes().await?.to_vec();
        Ok(bytes)
    }
}
