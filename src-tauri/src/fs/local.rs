use super::traits::{DriveInfo, FileItem, FileSystemProvider};
use async_trait::async_trait;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;

pub struct LocalFileSystem;

impl LocalFileSystem {
    pub fn new() -> Self {
        Self
    }

    pub fn list_drives() -> Vec<DriveInfo> {
        let mut drives = Vec::new();

        #[cfg(target_os = "windows")]
        {
            for drive_letter in b'A'..=b'Z' {
                let drive_str = format!("{}:\\", drive_letter as char);
                let path = Path::new(&drive_str);
                if path.exists() {
                    drives.push(DriveInfo {
                        name: format!("Disk ({}:)", drive_letter as char),
                        path: drive_str,
                        total_space: 0,
                        available_space: 0,
                        is_removable: drive_letter == b'A' || drive_letter == b'B',
                    });
                }
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            drives.push(DriveInfo {
                name: "Root (/)".to_string(),
                path: "/".to_string(),
                total_space: 0,
                available_space: 0,
                is_removable: false,
            });

            if let Ok(home) = std::env::var("HOME") {
                drives.push(DriveInfo {
                    name: "Home (~)".to_string(),
                    path: home,
                    total_space: 0,
                    available_space: 0,
                    is_removable: false,
                });
            }

            if Path::new("/media").exists() {
                if let Ok(entries) = fs::read_dir("/media") {
                    for entry in entries.flatten() {
                        if entry.path().is_dir() {
                            drives.push(DriveInfo {
                                name: format!("Media: {}", entry.file_name().to_string_lossy()),
                                path: entry.path().to_string_lossy().to_string(),
                                total_space: 0,
                                available_space: 0,
                                is_removable: true,
                            });
                        }
                    }
                }
            }
        }

        drives
    }
}

#[async_trait]
impl FileSystemProvider for LocalFileSystem {
    async fn list_directory(&self, path_str: &str) -> Result<Vec<FileItem>, Box<dyn Error + Send + Sync>> {
        let target_path = if path_str.is_empty() {
            std::env::current_dir().unwrap_or_else(|_| Path::new("/").to_path_buf())
        } else {
            Path::new(path_str).to_path_buf()
        };

        let mut items = Vec::new();

        // Add parent directory ".." entry if not at root
        if let Some(parent) = target_path.parent() {
            items.push(FileItem {
                name: "..".to_string(),
                path: parent.to_string_lossy().to_string(),
                size: 0,
                is_dir: true,
                modified: 0,
                permissions: "drwxr-xr-x".to_string(),
                is_symlink: false,
                is_hidden: false,
            });
        }

        let read_dir = match tokio::fs::read_dir(&target_path).await {
            Ok(rd) => rd,
            Err(e) => return Err(Box::new(e)),
        };

        let mut entries = read_dir;
        while let Ok(Some(entry)) = entries.next_entry().await {
            let metadata = match entry.metadata().await {
                Ok(m) => m,
                Err(_) => continue,
            };

            let file_name = entry.file_name().to_string_lossy().to_string();
            let is_hidden = file_name.starts_with('.');
            let is_dir = metadata.is_dir();
            let is_symlink = metadata.is_symlink();
            let size = if is_dir { 0 } else { metadata.len() };

            let modified = metadata
                .modified()
                .ok()
                .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                .map(|d| d.as_secs())
                .unwrap_or(0);

            let permissions = if is_dir {
                "drwxr-xr-x".to_string()
            } else {
                "-rw-r--r--".to_string()
            };

            items.push(FileItem {
                name: file_name,
                path: entry.path().to_string_lossy().to_string(),
                size,
                is_dir,
                modified,
                permissions,
                is_symlink,
                is_hidden,
            });
        }

        // Sort: Directory first (except .. at index 0), then alphabetically
        items.sort_by(|a, b| {
            if a.name == ".." {
                std::cmp::Ordering::Less
            } else if b.name == ".." {
                std::cmp::Ordering::Greater
            } else if a.is_dir != b.is_dir {
                b.is_dir.cmp(&a.is_dir)
            } else {
                a.name.to_lowercase().cmp(&b.name.to_lowercase())
            }
        });

        Ok(items)
    }

    async fn create_dir(&self, path: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        tokio::fs::create_dir_all(path).await?;
        Ok(())
    }

    async fn remove(&self, path: &str, is_dir: bool) -> Result<(), Box<dyn Error + Send + Sync>> {
        if is_dir {
            tokio::fs::remove_dir_all(path).await?;
        } else {
            tokio::fs::remove_file(path).await?;
        }
        Ok(())
    }

    async fn exists(&self, path: &str) -> Result<bool, Box<dyn Error + Send + Sync>> {
        Ok(Path::new(path).exists())
    }

    async fn read_file_preview(&self, path: &str, max_bytes: usize) -> Result<String, Box<dyn Error + Send + Sync>> {
        use tokio::io::AsyncReadExt;
        let mut file = tokio::fs::File::open(path).await?;
        let mut buffer = vec![0u8; max_bytes];
        let bytes_read = file.read(&mut buffer).await?;
        let text = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
        Ok(text)
    }
}
