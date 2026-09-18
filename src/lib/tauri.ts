import type { DriveInfo, FileItem, SftpConfig } from "./types";

// Helper to safely invoke Tauri IPC or fallback mock data when running in browser
async function safeInvoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (typeof window !== "undefined" && "__TAURI_INTERNALS__" in window) {
    const { invoke } = await import("@tauri-apps/api/core");
    return invoke<T>(cmd, args);
  } else {
    // Mock data fallback for browser preview
    console.warn(`[Tauri Mock] Invoking command: ${cmd}`, args);
    if (cmd === "list_drives") {
      return [
        { name: "Root (/)", path: "/", total_space: 1000000000, available_space: 500000000, is_removable: false },
        { name: "Home (~)", path: "/home/user", total_space: 500000000, available_space: 250000000, is_removable: false }
      ] as unknown as T;
    }
    if (cmd === "list_directory") {
      const path = (args?.path as string) || "/home/user";
      return [
        { name: "..", path: "/home", size: 0, is_dir: true, modified: Date.now() / 1000, permissions: "drwxr-xr-x", is_symlink: false, is_hidden: false },
        { name: "Documents", path: `${path}/Documents`, size: 0, is_dir: true, modified: Date.now() / 1000, permissions: "drwxr-xr-x", is_symlink: false, is_hidden: false },
        { name: "Downloads", path: `${path}/Downloads`, size: 0, is_dir: true, modified: Date.now() / 1000, permissions: "drwxr-xr-x", is_symlink: false, is_hidden: false },
        { name: "project.rs", path: `${path}/project.rs`, size: 4096, is_dir: false, modified: Date.now() / 1000, permissions: "-rw-r--r--", is_symlink: false, is_hidden: false },
        { name: "README.md", path: `${path}/README.md`, size: 1240, is_dir: false, modified: Date.now() / 1000, permissions: "-rw-r--r--", is_symlink: false, is_hidden: false },
      ] as unknown as T;
    }
    if (cmd === "read_file_text") {
      return "# RLCommander File Preview\n\nThis is a sample file content for demonstration." as unknown as T;
    }
    return Promise.resolve({} as T);
  }
}

export async function listDirectory(path: string, sftpConfig?: SftpConfig): Promise<FileItem[]> {
  return safeInvoke<FileItem[]>("list_directory", { path, sftpConfig });
}

export async function listDrives(): Promise<DriveInfo[]> {
  return safeInvoke<DriveInfo[]>("list_drives");
}

export async function createDirectory(path: string, sftpConfig?: SftpConfig): Promise<void> {
  return safeInvoke<void>("create_directory", { path, sftpConfig });
}

export async function deleteItems(items: Array<[string, boolean]>, sftpConfig?: SftpConfig): Promise<void> {
  return safeInvoke<void>("delete_items", { items, sftpConfig });
}

export async function readFileText(path: string, maxBytes?: number, sftpConfig?: SftpConfig): Promise<string> {
  return safeInvoke<string>("read_file_text", { path, maxBytes, sftpConfig });
}

export async function copyItemsAsync(
  srcPaths: string[],
  destDir: string,
  isMove: boolean,
  sftpConfig?: SftpConfig
): Promise<string> {
  return safeInvoke<string>("copy_items_async", { srcPaths, destDir, isMove, sftpConfig });
}
