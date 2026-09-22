use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

/// Compresses files/directories recursively into a .zip archive.
pub fn zip_items(src_paths: Vec<String>, dest_zip: String) -> Result<(), String> {
    let dest_path = Path::new(&dest_zip);
    if let Some(parent) = dest_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed to create output directory: {}", e))?;
        }
    }

    let file = fs::File::create(dest_path).map_err(|e| format!("Failed to create zip file: {}", e))?;
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);

    for src_str in src_paths {
        let src_path = Path::new(&src_str);
        if !src_path.exists() {
            return Err(format!("Source path does not exist: {}", src_str));
        }

        let base_dir = src_path.parent().unwrap_or_else(|| Path::new(""));
        add_path_to_zip(&mut zip, src_path, base_dir, options)?;
    }

    zip.finish().map_err(|e| format!("Failed to finalize zip archive: {}", e))?;
    Ok(())
}

fn add_path_to_zip<W: Write + std::io::Seek>(
    zip: &mut ZipWriter<W>,
    src_path: &Path,
    base_dir: &Path,
    options: SimpleFileOptions,
) -> Result<(), String> {
    let metadata = fs::symlink_metadata(src_path).map_err(|e| format!("Failed to read metadata for {:?}: {}", src_path, e))?;

    let rel_path = src_path
        .strip_prefix(base_dir)
        .map_err(|e| format!("Failed to compute relative path: {}", e))?;

    let name_in_zip = rel_path
        .components()
        .map(|c| c.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/");

    if metadata.is_dir() {
        if !name_in_zip.is_empty() {
            let dir_name = if name_in_zip.ends_with('/') {
                name_in_zip
            } else {
                format!("{}/", name_in_zip)
            };
            zip.add_directory(&dir_name, options)
                .map_err(|e| format!("Failed to add directory to zip: {}", e))?;
        }

        let entries = fs::read_dir(src_path).map_err(|e| format!("Failed to read dir {:?}: {}", src_path, e))?;
        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            add_path_to_zip(zip, &entry.path(), base_dir, options)?;
        }
    } else if metadata.is_file() {
        zip.start_file(&name_in_zip, options)
            .map_err(|e| format!("Failed to start file in zip: {}", e))?;
        let mut f = fs::File::open(src_path).map_err(|e| format!("Failed to open {:?}: {}", src_path, e))?;
        let mut buffer = vec![0u8; 64 * 1024];
        loop {
            let n = f.read(&mut buffer).map_err(|e| format!("Failed to read file {:?}: {}", src_path, e))?;
            if n == 0 {
                break;
            }
            zip.write_all(&buffer[..n]).map_err(|e| format!("Failed to write file to zip: {}", e))?;
        }
    }

    Ok(())
}

/// Extracts a .zip archive into dest_dir.
pub fn unzip_archive(zip_path: String, dest_dir: String) -> Result<(), String> {
    let file = fs::File::open(&zip_path).map_err(|e| format!("Failed to open zip file: {}", e))?;
    let mut archive = ZipArchive::new(file).map_err(|e| format!("Invalid zip archive: {}", e))?;

    let dest_path = Path::new(&dest_dir);
    fs::create_dir_all(dest_path).map_err(|e| format!("Failed to create destination directory: {}", e))?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| format!("Failed to read archive entry {}: {}", i, e))?;
        let outpath = match file.enclosed_name() {
            Some(path) => dest_path.join(path),
            None => continue,
        };

        if file.is_dir() || file.name().ends_with('/') {
            fs::create_dir_all(&outpath).map_err(|e| format!("Failed to create dir {:?}: {}", outpath, e))?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).map_err(|e| format!("Failed to create parent dir {:?}: {}", p, e))?;
                }
            }
            let mut outfile = fs::File::create(&outpath).map_err(|e| format!("Failed to create file {:?}: {}", outpath, e))?;
            std::io::copy(&mut file, &outfile).map_err(|e| format!("Failed to extract file {:?}: {}", outpath, e))?;
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                let _ = fs::set_permissions(&outpath, fs::Permissions::from_mode(mode));
            }
        }
    }

    Ok(())
}
