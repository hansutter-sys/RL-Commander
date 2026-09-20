use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileItem {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub is_dir: bool,
    pub modified: u64, // Unix timestamp in seconds
    pub permissions: String,
    pub is_symlink: bool,
    pub is_hidden: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriveInfo {
    pub name: String,
    pub path: String,
    pub total_space: u64,
    pub available_space: u64,
    pub is_removable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SftpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: Option<String>,
    pub private_key_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferProgressEvent {
    pub task_id: String,
    pub src_path: String,
    pub dest_path: String,
    pub bytes_transferred: u64,
    pub total_bytes: u64,
    pub speed_bytes_per_sec: u64,
    pub is_finished: bool,
    pub error: Option<String>,
}

#[async_trait]
pub trait FileSystemProvider: Send + Sync {
    async fn list_directory(&self, path: &str) -> Result<Vec<FileItem>, Box<dyn Error + Send + Sync>>;
    async fn create_dir(&self, path: &str) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn remove(&self, path: &str, is_dir: bool) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn exists(&self, path: &str) -> Result<bool, Box<dyn Error + Send + Sync>>;
    async fn read_file_preview(&self, path: &str, max_bytes: usize) -> Result<String, Box<dyn Error + Send + Sync>>;
    async fn write_file(&self, path: &str, content: &str) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn search_files(&self, base_path: &str, query: &str) -> Result<Vec<FileItem>, Box<dyn Error + Send + Sync>>;
}
