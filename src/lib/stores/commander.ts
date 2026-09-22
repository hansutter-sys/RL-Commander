import { writable, derived, get } from "svelte/store";
import type { PaneId, PaneState, TransferProgressEvent, SftpConfig, Bookmark } from "../types";
import { listDirectory, listDrives, createDirectory, deleteItems, copyItemsAsync } from "../tauri";
import { settings } from "./settings";

export const activePane = writable<PaneId>("left");

const initialPaneState = (id: PaneId, path: string): PaneState => ({
  id,
  currentPath: path,
  files: [],
  selectedIndex: 0,
  selectedPaths: new Set<string>(),
  sortColumn: "name",
  sortAscending: true,
  isSftp: false,
  loading: false,
  error: null,
  quickFilter: "",
});

const initialSettings = get(settings);

export const leftPane = writable<PaneState>(initialPaneState("left", initialSettings.defaultLeftPath || "/"));
export const rightPane = writable<PaneState>(initialPaneState("right", initialSettings.defaultRightPath || "/home"));

export const activePaneState = derived([activePane, leftPane, rightPane], ([$active, $left, $right]) =>
  $active === "left" ? $left : $right
);

export const inactivePaneState = derived([activePane, leftPane, rightPane], ([$active, $left, $right]) =>
  $active === "left" ? $right : $left
);

export const activeTransfer = writable<TransferProgressEvent | null>(null);

// Modal states
export const showSftpModal = writable<boolean>(false);
export const showViewerModal = writable<boolean>(false);
export const showEditorModal = writable<boolean>(false);
export const showSearchModal = writable<boolean>(false);
export const showCreateDirModal = writable<boolean>(false);
export const showDeleteModal = writable<boolean>(false);
export const showSettingsModal = writable<boolean>(false);
export const showZipModal = writable<boolean>(false);

export const zipSourcePaths = writable<string[]>([]);
export const zipDefaultName = writable<string>("archive.zip");

export function openZipModal(paths: string[], defaultName?: string) {
  zipSourcePaths.set(paths);
  if (defaultName) {
    zipDefaultName.set(defaultName);
  } else if (paths.length === 1) {
    const filename = paths[0].split("/").pop() || "archive";
    const nameWithoutExt = filename.replace(/\.[^/.]+$/, "");
    zipDefaultName.set(`${nameWithoutExt}.zip`);
  } else {
    zipDefaultName.set("archive.zip");
  }
  showZipModal.set(true);
}

export const viewerFilePath = writable<string>("");
export const editorFilePath = writable<string>("");

const DEFAULT_BOOKMARKS: Bookmark[] = [
  { name: "Root (/)", path: "/", icon: "HardDrive" },
  { name: "Hem (~)", path: "/home", icon: "Home" },
  { name: "Dokument", path: "/home/Dokument", icon: "Folder" },
  { name: "Hämtningar", path: "/home/Hämtningar", icon: "Download" },
  { name: "Projekt", path: "/home/hans/RL-Commander", icon: "Code" },
  { name: "/tmp", path: "/tmp", icon: "FolderArchive" },
  { name: "/etc", path: "/etc", icon: "Settings" },
];

function loadBookmarks(): Bookmark[] {
  if (typeof localStorage !== "undefined") {
    try {
      const stored = localStorage.getItem("rl_commander_bookmarks");
      if (stored) {
        return JSON.parse(stored);
      }
    } catch (e) {}
  }
  return DEFAULT_BOOKMARKS;
}

export const bookmarks = writable<Bookmark[]>(loadBookmarks());

bookmarks.subscribe((value) => {
  if (typeof localStorage !== "undefined") {
    try {
      localStorage.setItem("rl_commander_bookmarks", JSON.stringify(value));
    } catch (e) {}
  }
});

export function addBookmark(path: string, name?: string) {
  bookmarks.update((bms) => {
    if (bms.some((b) => b.path === path)) return bms;
    const folderName = name || path.split("/").filter(Boolean).pop() || path;
    return [...bms, { name: folderName, path, icon: "Folder" }];
  });
}

export function removeBookmark(path: string) {
  bookmarks.update((bms) => bms.filter((b) => b.path !== path));
}

export function toggleActivePane() {
  activePane.update((curr) => (curr === "left" ? "right" : "left"));
}

export async function refreshPane(paneId: PaneId) {
  const store = paneId === "left" ? leftPane : rightPane;
  const state = get(store);

  store.update((s) => ({ ...s, loading: true, error: null }));
  try {
    const files = await listDirectory(state.currentPath, state.sftpConfig);
    store.update((s) => ({
      ...s,
      files,
      selectedIndex: Math.min(s.selectedIndex, Math.max(0, files.length - 1)),
      loading: false,
    }));
  } catch (err: any) {
    store.update((s) => ({
      ...s,
      loading: false,
      error: err?.toString() || "Failed to list directory",
    }));
  }
}

export function navigatePane(paneId: PaneId, newPath: string) {
  const store = paneId === "left" ? leftPane : rightPane;
  store.update((s) => ({ ...s, currentPath: newPath, selectedIndex: 0, selectedPaths: new Set() }));
  refreshPane(paneId);
}

export function toggleSelection(paneId: PaneId, path: string) {
  const store = paneId === "left" ? leftPane : rightPane;
  store.update((s) => {
    const newSet = new Set(s.selectedPaths);
    if (newSet.has(path)) {
      newSet.delete(path);
    } else {
      newSet.add(path);
    }
    return { ...s, selectedPaths: newSet };
  });
}

export function connectSftp(paneId: PaneId, config: SftpConfig) {
  const store = paneId === "left" ? leftPane : rightPane;
  store.update((s) => ({
    ...s,
    isSftp: true,
    sftpConfig: config,
    currentPath: "/",
  }));
  refreshPane(paneId);
}

