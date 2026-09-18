use crate::fs::{
    DriveInfo, FileItem, FileSystemProvider, LocalFileSystem, SftpConfig, SftpFileSystem, TransferProgressEvent,
};
use std::path::Path;
use std::time::Instant;
use tauri::{AppHandle, Emitter};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tauri::command]
pub async fn list_directory(
    path: String,
    sftp_config: Option<SftpConfig>,
) -> Result<Vec<FileItem>, String> {
    if let Some(config) = sftp_config {
        let sftp_fs = SftpFileSystem::new(config);
        sftp_fs.list_directory(&path).await.map_err(|e| e.to_string())
    } else {
        let local_fs = LocalFileSystem::new();
        local_fs.list_directory(&path).await.map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub async fn list_drives() -> Result<Vec<DriveInfo>, String> {
    Ok(LocalFileSystem::list_drives())
}

#[tauri::command]
pub async fn create_directory(
    path: String,
    sftp_config: Option<SftpConfig>,
) -> Result<(), String> {
    if let Some(config) = sftp_config {
        let sftp_fs = SftpFileSystem::new(config);
        sftp_fs.create_dir(&path).await.map_err(|e| e.to_string())
    } else {
        let local_fs = LocalFileSystem::new();
        local_fs.create_dir(&path).await.map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub async fn delete_items(
    items: Vec<(String, bool)>, // (path, is_dir)
    sftp_config: Option<SftpConfig>,
) -> Result<(), String> {
    for (path, is_dir) in items {
        if let Some(ref config) = sftp_config {
            let sftp_fs = SftpFileSystem::new(config.clone());
            sftp_fs.remove(&path, is_dir).await.map_err(|e| e.to_string())?;
        } else {
            let local_fs = LocalFileSystem::new();
            local_fs.remove(&path, is_dir).await.map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn read_file_text(
    path: String,
    max_bytes: Option<usize>,
    sftp_config: Option<SftpConfig>,
) -> Result<String, String> {
    let limit = max_bytes.unwrap_or(200_000);
    if let Some(config) = sftp_config {
        let sftp_fs = SftpFileSystem::new(config);
        sftp_fs.read_file_preview(&path, limit).await.map_err(|e| e.to_string())
    } else {
        let local_fs = LocalFileSystem::new();
        local_fs.read_file_preview(&path, limit).await.map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub async fn copy_items_async(
    app: AppHandle,
    src_paths: Vec<String>,
    dest_dir: String,
    is_move: bool,
    sftp_config: Option<SftpConfig>,
) -> Result<String, String> {
    let task_id = format!("task_{}", chrono::Utc::now().timestamp_millis());
    let task_id_clone = task_id.clone();

    tokio::spawn(async move {
        for src_str in src_paths {
            let src_path = Path::new(&src_str);
            let file_name = match src_path.file_name() {
                Some(name) => name,
                None => continue,
            };
            let dest_path_buf = Path::new(&dest_dir).join(file_name);
            let dest_str = dest_path_buf.to_string_lossy().to_string();

            // Local-to-Local copy stream
            if sftp_config.is_none() {
                let mut src_file = match tokio::fs::File::open(&src_str).await {
                    Ok(f) => f,
                    Err(e) => {
                        let _ = app.emit(
                            "transfer-progress",
                            TransferProgressEvent {
                                task_id: task_id_clone.clone(),
                                src_path: src_str.clone(),
                                dest_path: dest_str,
                                bytes_transferred: 0,
                                total_bytes: 0,
                                speed_bytes_per_sec: 0,
                                is_finished: true,
                                error: Some(e.to_string()),
                            },
                        );
                        continue;
                    }
                };

                let total_bytes = src_file.metadata().await.map(|m| m.len()).unwrap_or(0);
                let mut dest_file = match tokio::fs::File::create(&dest_str).await {
                    Ok(f) => f,
                    Err(e) => {
                        let _ = app.emit(
                            "transfer-progress",
                            TransferProgressEvent {
                                task_id: task_id_clone.clone(),
                                src_path: src_str.clone(),
                                dest_path: dest_str,
                                bytes_transferred: 0,
                                total_bytes,
                                speed_bytes_per_sec: 0,
                                is_finished: true,
                                error: Some(e.to_string()),
                            },
                        );
                        continue;
                    }
                };

                let mut buffer = vec![0u8; 64 * 1024]; // 64 KB chunking
                let mut bytes_transferred = 0u64;
                let start_time = Instant::now();

                loop {
                    let n = match src_file.read(&mut buffer).await {
                        Ok(0) => break,
                        Ok(n) => n,
                        Err(e) => {
                            let _ = app.emit(
                                "transfer-progress",
                                TransferProgressEvent {
                                    task_id: task_id_clone.clone(),
                                    src_path: src_str.clone(),
                                    dest_path: dest_str.clone(),
                                    bytes_transferred,
                                    total_bytes,
                                    speed_bytes_per_sec: 0,
                                    is_finished: true,
                                    error: Some(e.to_string()),
                                },
                            );
                            break;
                        }
                    };

                    if let Err(e) = dest_file.write_all(&buffer[..n]).await {
                        let _ = app.emit(
                            "transfer-progress",
                            TransferProgressEvent {
                                task_id: task_id_clone.clone(),
                                src_path: src_str.clone(),
                                dest_path: dest_str.clone(),
                                bytes_transferred,
                                total_bytes,
                                speed_bytes_per_sec: 0,
                                is_finished: true,
                                error: Some(e.to_string()),
                            },
                        );
                        break;
                    }

                    bytes_transferred += n as u64;
                    let elapsed = start_time.elapsed().as_secs_f64();
                    let speed = if elapsed > 0.0 {
                        (bytes_transferred as f64 / elapsed) as u64
                    } else {
                        0
                    };

                    let _ = app.emit(
                        "transfer-progress",
                        TransferProgressEvent {
                            task_id: task_id_clone.clone(),
                            src_path: src_str.clone(),
                            dest_path: dest_str.clone(),
                            bytes_transferred,
                            total_bytes,
                            speed_bytes_per_sec: speed,
                            is_finished: bytes_transferred >= total_bytes,
                            error: None,
                        },
                    );
                }

                if is_move && bytes_transferred >= total_bytes {
                    let _ = tokio::fs::remove_file(&src_str).await;
                }
            }
        }
    });

    Ok(task_id)
}
