<script lang="ts">
  import type { FileItem, PaneId } from "../types";
  import ContextMenu from "./ContextMenu.svelte";
  import { Folder, FileCode, FileText, Image, Archive, Terminal } from "lucide-svelte";
  import { draggedItems } from "../stores/commander";
  import { settings } from "../stores/settings";

  export let files: FileItem[] = [];
  export let selectedIndex: number = 0;
  export let selectedPaths: Set<string> = new Set();
  export let isActive: boolean = false;
  export let paneId: PaneId;

  export let onSelect: (index: number) => void;
  export let onOpen: (item: FileItem) => void;
  export let onToggleSelection: (item: FileItem) => void;
  export let onRangeSelect: (fromIndex: number, toIndex: number) => void = () => {};
  export let onDropToPath: (e: DragEvent, targetPath: string) => void = () => {};

  export let onView: (item: FileItem) => void;
  export let onEdit: (item: FileItem) => void;
  export let onCopy: (item: FileItem) => void;
  export let onMove: (item: FileItem) => void;
  export let onDelete: (item: FileItem) => void;
  export let onZip: (item: FileItem) => void;
  export let onUnzip: (item: FileItem) => void;

  let contextMenuVisible = false;
  let contextMenuX = 0;
  let contextMenuY = 0;
  let contextMenuItem: FileItem | null = null;
  let dragOverItemPath: string | null = null;

  function formatBytes(bytes: number): string {
    if (bytes === 0) return "<DIR>";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + " " + sizes[i];
  }

  function formatDate(timestamp: number): string {
    if (!timestamp) return "-";
    const date = new Date(timestamp * 1000);
    return date.toLocaleDateString("sv-SE") + " " + date.toLocaleTimeString("sv-SE", { hour: "2-digit", minute: "2-digit" });
  }

  function getFileIcon(item: FileItem) {
    if (item.is_dir) return Folder;
    const ext = item.name.split(".").pop()?.toLowerCase();
    if (["rs", "ts", "js", "py", "html", "css", "json"].includes(ext || "")) return FileCode;
    if (["png", "jpg", "jpeg", "svg", "webp"].includes(ext || "")) return Image;
    if (["zip", "tar", "gz", "7z"].includes(ext || "")) return Archive;
    if (["sh", "bash", "exe"].includes(ext || "")) return Terminal;
    return FileText;
  }

  function handleContextMenu(e: MouseEvent, item: FileItem, index: number) {
    if (item.name === "..") return;
    e.preventDefault();
    e.stopPropagation();
    onSelect(index);
    contextMenuX = e.clientX;
    contextMenuY = e.clientY;
    contextMenuItem = item;
    contextMenuVisible = true;
  }

  function handleRowClick(e: MouseEvent, item: FileItem, index: number) {
    if (e.ctrlKey || e.metaKey) {
      // Ctrl + Click: toggle selection
      if (item.name !== "..") {
        onToggleSelection(item);
      }
      onSelect(index);
    } else if (e.shiftKey) {
      // Shift + Click: range select from selectedIndex to index
      onRangeSelect(selectedIndex, index);
    } else {
      // Regular click: select row
      onSelect(index);
    }
  }

  function handleAuxClick(e: MouseEvent, item: FileItem, index: number) {
    // Middle click (scroll-click): toggle selection
    if (e.button === 1 && item.name !== "..") {
      e.preventDefault();
      onToggleSelection(item);
      onSelect(index);
    }
  }

  function handleDragStart(e: DragEvent, item: FileItem) {
    if (item.name === "..") {
      e.preventDefault();
      return;
    }

    let pathsToDrag = Array.from(selectedPaths);
    if (!selectedPaths.has(item.path) || pathsToDrag.length === 0) {
      pathsToDrag = [item.path];
    }

    draggedItems.set({
      paths: pathsToDrag,
      sourcePaneId: paneId,
    });

    if (e.dataTransfer) {
      e.dataTransfer.effectAllowed = "copyMove";
      e.dataTransfer.setData(
        "application/json",
        JSON.stringify({
          paths: pathsToDrag,
          sourcePaneId: paneId,
        })
      );
      e.dataTransfer.setData("text/plain", JSON.stringify(pathsToDrag));
    }
  }

  function handleDragEnd() {
    dragOverItemPath = null;
    draggedItems.set(null);
  }

  function handleRowDragOver(e: DragEvent, item: FileItem) {
    if (item.is_dir && item.name !== "..") {
      e.preventDefault();
      e.stopPropagation();
      if (e.dataTransfer) {
        e.dataTransfer.dropEffect = "copy";
      }
      dragOverItemPath = item.path;
    }
  }

  function handleRowDragLeave(e: DragEvent, item: FileItem) {
    if (dragOverItemPath === item.path) {
      dragOverItemPath = null;
    }
  }

  function handleRowDrop(e: DragEvent, item: FileItem) {
    if (item.is_dir && item.name !== "..") {
      e.preventDefault();
      e.stopPropagation();
      dragOverItemPath = null;
      onDropToPath(e, item.path);
    }
  }
</script>

<div
  class="flex-1 overflow-y-auto text-sm mono-font border-t relative transition-colors duration-150
    {$settings.theme === 'light' ? 'bg-white border-slate-200 text-slate-800' : 'bg-slate-900/90 border-slate-800 text-slate-200'}"
>
  <table class="w-full text-left border-collapse">
    <thead
      class="sticky top-0 font-semibold border-b text-xs select-none z-10 transition-colors
        {$settings.theme === 'light' ? 'bg-slate-100 text-slate-600 border-slate-300' : 'bg-slate-950 text-slate-400 border-slate-800'}"
    >
      <tr>
        <th class="w-8 px-2 py-1.5 text-center">✓</th>
        <th class="px-3 py-1.5">Namn</th>
        <th class="w-24 px-3 py-1.5 text-right">Storlek</th>
        <th class="w-36 px-3 py-1.5 text-right">Ändrad</th>
        <th class="w-28 px-3 py-1.5 text-center hidden md:table-cell">Rättigheter</th>
      </tr>
    </thead>
    <tbody class="{$settings.theme === 'light' ? 'divide-y divide-slate-200/80' : 'divide-y divide-slate-800/50'}">
      {#each files as item, index (item.path + "_" + index)}
        <tr
          draggable={item.name !== ".."}
          on:dragstart={(e) => handleDragStart(e, item)}
          on:dragend={handleDragEnd}
          on:dragover={(e) => handleRowDragOver(e, item)}
          on:dragleave={(e) => handleRowDragLeave(e, item)}
          on:drop={(e) => handleRowDrop(e, item)}
          on:contextmenu={(e) => handleContextMenu(e, item, index)}
          on:click={(e) => handleRowClick(e, item, index)}
          on:auxclick={(e) => handleAuxClick(e, item, index)}
          on:dblclick={() => onOpen(item)}
          class="cursor-pointer transition-colors duration-75 text-xs font-mono select-none
            {dragOverItemPath === item.path
              ? 'bg-emerald-600 text-white font-bold ring-2 ring-emerald-400'
              : index === selectedIndex && isActive
              ? 'bg-blue-600 text-white font-bold shadow-sm'
              : index === selectedIndex && !isActive
              ? ($settings.theme === 'light' ? 'bg-blue-50 text-slate-900 border-l-2 border-blue-600 font-semibold' : 'bg-slate-800 text-slate-100 border-l-2 border-blue-500')
              : selectedPaths.has(item.path)
              ? ($settings.theme === 'light' ? 'bg-amber-100/90 text-amber-900 font-semibold' : 'bg-amber-950/30 text-amber-300 font-semibold')
              : ($settings.theme === 'light' ? 'hover:bg-slate-100 text-slate-700' : 'hover:bg-slate-800/60 text-slate-300')}"
        >
          <td
            class="w-8 px-2 py-1 text-center"
            title="Klicka för att markera (Ctrl+klick / scrollklick fungerar också)"
            on:click|stopPropagation={() => onToggleSelection(item)}
          >
            {#if selectedPaths.has(item.path)}
              <span class="text-amber-500 font-bold">★</span>
            {:else if item.is_dir}
              <span class="{$settings.theme === 'light' ? 'text-slate-400' : 'text-slate-600'}">▪</span>
            {:else}
              <span class="{$settings.theme === 'light' ? 'text-slate-300' : 'text-slate-700'}">▫</span>
            {/if}
          </td>

          <td class="px-3 py-1 flex items-center gap-2 truncate max-w-xs">
            <svelte:component
              this={getFileIcon(item)}
              class="w-4 h-4 flex-shrink-0 {item.is_dir ? 'text-sky-500' : item.name.endsWith('.rs') ? 'text-orange-500' : ($settings.theme === 'light' ? 'text-slate-500' : 'text-slate-400')}"
            />
            <span class="truncate {item.is_dir ? ($settings.theme === 'light' ? 'text-sky-700 font-bold' : 'text-sky-300 font-semibold') : ''}">
              {item.name}
            </span>
          </td>

          <td class="w-24 px-3 py-1 text-right font-mono text-xs">
            {formatBytes(item.size)}
          </td>

          <td class="w-36 px-3 py-1 text-right text-xs {$settings.theme === 'light' ? 'text-slate-500' : 'text-slate-400'}">
            {formatDate(item.modified)}
          </td>

          <td class="w-28 px-3 py-1 text-center text-xs {$settings.theme === 'light' ? 'text-slate-400' : 'text-slate-500'} hidden md:table-cell">
            {item.permissions}
          </td>
        </tr>
      {/each}
      {#if files.length === 0}
        <tr>
          <td colspan="5" class="px-4 py-8 text-center {$settings.theme === 'light' ? 'text-slate-400' : 'text-slate-500'} italic">
            Mappen är tom
          </td>
        </tr>
      {/if}
    </tbody>
  </table>

  <!-- Right click Context Menu -->
  <ContextMenu
    x={contextMenuX}
    y={contextMenuY}
    visible={contextMenuVisible}
    item={contextMenuItem}
    {paneId}
    onClose={() => (contextMenuVisible = false)}
    onZip={(item) => onZip(item)}
    onUnzip={(item) => onUnzip(item)}
    onView={(item) => onView(item)}
    onEdit={(item) => onEdit(item)}
    onCopy={(item) => onCopy(item)}
    onMove={(item) => onMove(item)}
    onDelete={(item) => onDelete(item)}
  />
</div>
