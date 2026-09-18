export interface FileItem {
  name: string;
  path: string;
  size: number;
  is_dir: boolean;
  modified: number;
  permissions: string;
  is_symlink: boolean;
  is_hidden: boolean;
}

export interface DriveInfo {
  name: string;
  path: string;
  total_space: number;
  available_space: number;
  is_removable: boolean;
}

export interface SftpConfig {
  host: string;
  port: number;
  username: string;
  password?: string;
  private_key_path?: string;
}

export interface TransferProgressEvent {
  task_id: string;
  src_path: string;
  dest_path: string;
  bytes_transferred: number;
  total_bytes: number;
  speed_bytes_per_sec: number;
  is_finished: boolean;
  error?: string;
}

export type PaneId = "left" | "right";

export interface PaneState {
  id: PaneId;
  currentPath: string;
  files: FileItem[];
  selectedIndex: number;
  selectedPaths: Set<string>;
  sortColumn: "name" | "ext" | "size" | "modified";
  sortAscending: boolean;
  isSftp: boolean;
  sftpConfig?: SftpConfig;
  loading: boolean;
  error: string | null;
}
