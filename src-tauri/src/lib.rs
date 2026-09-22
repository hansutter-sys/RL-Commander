pub mod commands;
pub mod fs;

use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            list_directory,
            list_drives,
            create_directory,
            delete_items,
            read_file_text,
            write_file_text,
            search_files,
            copy_items_async,
            zip_items,
            unzip_archive,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
