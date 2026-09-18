use super::traits::{FileItem, FileSystemProvider, SftpConfig};
use async_trait::async_trait;
use ssh2::Session;
use std::error::Error;
use std::io::Read;
use std::net::TcpStream;
use std::path::Path;

pub struct SftpFileSystem {
    config: SftpConfig,
}

impl SftpFileSystem {
    pub fn new(config: SftpConfig) -> Self {
        Self { config }
    }

    fn connect(&self) -> Result<(Session, ssh2::Sftp), Box<dyn Error + Send + Sync>> {
        let addr = format!("{}:{}", self.config.host, self.config.port);
        let tcp = TcpStream::connect(&addr)?;
        let mut sess = Session::new()?;
        sess.set_tcp_stream(tcp);
        sess.handshake()?;

        if let Some(ref pass) = self.config.password {
            sess.userauth_password(&self.config.username, pass)?;
        } else if let Some(ref key_path) = self.config.private_key_path {
            sess.userauth_pubkey_file(&self.config.username, None, Path::new(key_path), None)?;
        } else {
            sess.userauth_agent(&self.config.username)?;
        }

        if !sess.authenticated() {
            return Err("SFTP authentication failed".into());
        }

        let sftp = sess.sftp()?;
        Ok((sess, sftp))
    }
}

#[async_trait]
impl FileSystemProvider for SftpFileSystem {
    async fn list_directory(&self, path_str: &str) -> Result<Vec<FileItem>, Box<dyn Error + Send + Sync>> {
        let path = if path_str.is_empty() { "." } else { path_str };
        let config_clone = self.config.clone();
        let path_buf = path.to_string();

        tokio::task::spawn_blocking(move || {
            let sftp_fs = SftpFileSystem::new(config_clone);
            let (_sess, sftp) = sftp_fs.connect()?;
            let target_path = Path::new(&path_buf);
            let entries = sftp.readdir(target_path)?;

            let mut items = Vec::new();
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

            for (entry_path, stat) in entries {
                let file_name = entry_path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                if file_name.is_empty() || file_name == "." || file_name == ".." {
                    continue;
                }

                let is_dir = stat.is_dir();
                let is_symlink = stat.is_symlink();
                let is_hidden = file_name.starts_with('.');
                let size = if is_dir { 0 } else { stat.size.unwrap_or(0) };
                let modified = stat.mtime.unwrap_or(0);

                let permissions = if is_dir {
                    "drwxr-xr-x".to_string()
                } else {
                    "-rw-r--r--".to_string()
                };

                items.push(FileItem {
                    name: file_name,
                    path: entry_path.to_string_lossy().to_string(),
                    size,
                    is_dir,
                    modified,
                    permissions,
                    is_symlink,
                    is_hidden,
                });
            }

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
        })
        .await?
    }

    async fn create_dir(&self, path: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        let config_clone = self.config.clone();
        let path_str = path.to_string();

        tokio::task::spawn_blocking(move || {
            let sftp_fs = SftpFileSystem::new(config_clone);
            let (_sess, sftp) = sftp_fs.connect()?;
            sftp.mkdir(Path::new(&path_str), 0o755)?;
            Ok(())
        })
        .await?
    }

    async fn remove(&self, path: &str, is_dir: bool) -> Result<(), Box<dyn Error + Send + Sync>> {
        let config_clone = self.config.clone();
        let path_str = path.to_string();

        tokio::task::spawn_blocking(move || {
            let sftp_fs = SftpFileSystem::new(config_clone);
            let (_sess, sftp) = sftp_fs.connect()?;
            if is_dir {
                sftp.rmdir(Path::new(&path_str))?;
            } else {
                sftp.unlink(Path::new(&path_str))?;
            }
            Ok(())
        })
        .await?
    }

    async fn exists(&self, path: &str) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let config_clone = self.config.clone();
        let path_str = path.to_string();

        tokio::task::spawn_blocking(move || {
            let sftp_fs = SftpFileSystem::new(config_clone);
            let (_sess, sftp) = sftp_fs.connect()?;
            Ok(sftp.stat(Path::new(&path_str)).is_ok())
        })
        .await?
    }

    async fn read_file_preview(&self, path: &str, max_bytes: usize) -> Result<String, Box<dyn Error + Send + Sync>> {
        let config_clone = self.config.clone();
        let path_str = path.to_string();

        tokio::task::spawn_blocking(move || {
            let sftp_fs = SftpFileSystem::new(config_clone);
            let (_sess, sftp) = sftp_fs.connect()?;
            let mut remote_file = sftp.open(Path::new(&path_str))?;
            let mut buffer = vec![0u8; max_bytes];
            let bytes_read = remote_file.read(&mut buffer)?;
            let text = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
            Ok(text)
        })
        .await?
    }
}
