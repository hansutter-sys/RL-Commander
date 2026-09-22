use super::traits::{DriveInfo, FileItem, FileSystemProvider};
use async_trait::async_trait;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;

pub struct LocalFileSystem;

#[cfg(unix)]
pub fn get_disk_space(path: &str) -> (u64, u64) {
    use std::ffi::CString;
    use std::mem::MaybeUninit;

    let c_path = match CString::new(path) {
        Ok(c) => c,
        Err(_) => return (0, 0),
    };

    unsafe {
        let mut stat: MaybeUninit<libc::statvfs> = MaybeUninit::uninit();
        if libc::statvfs(c_path.as_ptr(), stat.as_mut_ptr()) == 0 {
            let stat = stat.assume_init();
            let block_size = if stat.f_frsize > 0 {
                stat.f_frsize as u64
            } else {
                stat.f_bsize as u64
            };
            let total_space = stat.f_blocks as u64 * block_size;
            let available_space = stat.f_bavail as u64 * block_size;
            (total_space, available_space)
        } else {
            (0, 0)
        }
    }
}

#[cfg(windows)]
pub fn get_disk_space(path: &str) -> (u64, u64) {
    use std::os::windows::ffi::OsStrExt;
    use std::ffi::OsStr;

    let wide: Vec<u16> = OsStr::new(path).encode_wide().chain(std::iter::once(0)).collect();
    let mut free_bytes_available: u64 = 0;
    let mut total_bytes: u64 = 0;
    let mut total_free_bytes: u64 = 0;

    extern "system" {
        fn GetDiskFreeSpaceExW(
            lpDirectoryName: *const u16,
            lpFreeBytesAvailableToCaller: *mut u64,
            lpTotalNumberOfBytes: *mut u64,
            lpTotalNumberOfFreeBytes: *mut u64,
        ) -> i32;
    }

    unsafe {
        if GetDiskFreeSpaceExW(
            wide.as_ptr(),
            &mut free_bytes_available,
            &mut total_bytes,
            &mut total_free_bytes,
        ) != 0 {
            (total_bytes, free_bytes_available)
        } else {
            (0, 0)
        }
    }
}

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
                    let (total_space, available_space) = get_disk_space(&drive_str);
                    drives.push(DriveInfo {
                        name: format!("Disk ({}:)", drive_letter as char),
                        path: drive_str,
                        total_space,
                        available_space,
                        is_removable: drive_letter == b'A' || drive_letter == b'B',
                    });
                }
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            let (total_space, available_space) = get_disk_space("/");
            drives.push(DriveInfo {
                name: "Root (/)".to_string(),
                path: "/".to_string(),
                total_space,
                available_space,
                is_removable: false,
            });

            if let Ok(home) = std::env::var("HOME") {
                let (total_space, available_space) = get_disk_space(&home);
                drives.push(DriveInfo {
                    name: "Home (~)".to_string(),
                    path: home,
                    total_space,
                    available_space,
                    is_removable: false,
                });
            }

            if Path::new("/media").exists() {
                if let Ok(entries) = fs::read_dir("/media") {
                    for entry in entries.flatten() {
                        if entry.path().is_dir() {
                            let media_path = entry.path().to_string_lossy().to_string();
                            let (total_space, available_space) = get_disk_space(&media_path);
                            drives.push(DriveInfo {
                                name: format!("Media: {}", entry.file_name().to_string_lossy()),
                                path: media_path,
                                total_space,
                                available_space,
                                is_removable: true,
                            });
                        }
                    }
                }
            }
        }

        drives
    }

    pub fn sanitize_path(path_str: &str) -> std::path::PathBuf {
        let p = if path_str.is_empty() {
            std::env::current_dir().unwrap_or_else(|_| Path::new("/").to_path_buf())
        } else {
            Path::new(path_str).to_path_buf()
        };
        // Canonicalize if path exists to resolve symlinks and redundant relative components
        p.canonicalize().unwrap_or(p)
    }
}

#[async_trait]
impl FileSystemProvider for LocalFileSystem {
    async fn list_directory(&self, path_str: &str) -> Result<Vec<FileItem>, Box<dyn Error + Send + Sync>> {
        let target_path = LocalFileSystem::sanitize_path(path_str);

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

    async fn write_file(&self, path: &str, content: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        tokio::fs::write(path, content.as_bytes()).await?;
        Ok(())
    }

    async fn search_files(&self, base_path: &str, query: &str) -> Result<Vec<FileItem>, Box<dyn Error + Send + Sync>> {
        let query_lower = query.to_lowercase();
        let mut results = Vec::new();
        let mut stack = vec![(Path::new(base_path).to_path_buf(), 0)];

        while let Some((dir, depth)) = stack.pop() {
            if depth > 8 || results.len() >= 300 {
                continue;
            }
            let mut read_dir = match tokio::fs::read_dir(&dir).await {
                Ok(rd) => rd,
                Err(_) => continue,
            };
            while let Ok(Some(entry)) = read_dir.next_entry().await {
                let metadata = match entry.metadata().await {
                    Ok(m) => m,
                    Err(_) => continue,
                };
                let file_name = entry.file_name().to_string_lossy().to_string();
                let is_dir = metadata.is_dir();

                if file_name.to_lowercase().contains(&query_lower) {
                    let modified = metadata
                        .modified()
                        .ok()
                        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                        .map(|d| d.as_secs())
                        .unwrap_or(0);

                    results.push(FileItem {
                        name: file_name.clone(),
                        path: entry.path().to_string_lossy().to_string(),
                        size: if is_dir { 0 } else { metadata.len() },
                        is_dir,
                        modified,
                        permissions: if is_dir { "drwxr-xr-x".to_string() } else { "-rw-r--r--".to_string() },
                        is_symlink: metadata.is_symlink(),
                        is_hidden: file_name.starts_with('.'),
                    });
                }

                if is_dir && !file_name.starts_with('.') && file_name != "node_modules" && file_name != "target" {
                    stack.push((entry.path(), depth + 1));
                }
            }
        }

        Ok(results)
    }
}
