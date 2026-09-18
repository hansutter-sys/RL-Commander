<script lang="ts">
  import type { FileItem, PaneId } from "../types";
  import { Folder, FileCode, FileText, Image, Archive, HardDrive, Terminal, CheckCircle2 } from "lucide-svelte";

  export let files: FileItem[] = [];
  export let selectedIndex: number = 0;
  export let selectedPaths: Set<string> = new Set();
  export let isActive: boolean = false;
  export let paneId: PaneId;

  export let onSelect: (index: number) => void;
  export let onOpen: (item: FileItem) => void;
  export let onToggleSelection: (item: FileItem) => void;

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
</script>

<div class="flex-1 overflow-y-auto bg-slate-900/90 text-sm mono-font border-t border-slate-800">
  <table class="w-full text-left border-collapse">
    <thead class="sticky top-0 bg-slate-950 text-slate-400 font-semibold border-b border-slate-800 text-xs select-none">
      <tr>
        <th class="w-8 px-2 py-1.5 text-center">✓</th>
        <th class="px-3 py-1.5">Namn</th>
        <th class="w-24 px-3 py-1.5 text-right">Storlek</th>
        <th class="w-36 px-3 py-1.5 text-right">Ändrad</th>
        <th class="w-28 px-3 py-1.5 text-center hidden md:table-cell">Rättigheter</th>
      </tr>
    </thead>
    <tbody class="divide-y divide-slate-800/50">
      {#each files as item, index (item.path + "_" + index)}
        <tr
          class="cursor-pointer transition-colors duration-75 text-xs font-mono
            {index === selectedIndex && isActive ? 'bg-blue-600 text-white font-bold' : ''}
            {index === selectedIndex && !isActive ? 'bg-slate-800 text-slate-200 border-l-2 border-blue-500' : ''}
            {selectedPaths.has(item.path) && index !== selectedIndex ? 'text-amber-400 font-semibold bg-amber-950/20' : ''}
            {index !== selectedIndex && !selectedPaths.has(item.path) ? 'hover:bg-slate-800/60 text-slate-300' : ''}"
          on:click={() => onSelect(index)}
          on:dblclick={() => onOpen(item)}
        >
          <td class="w-8 px-2 py-1 text-center" on:click|stopPropagation={() => onToggleSelection(item)}>
            {#if selectedPaths.has(item.path)}
              <span class="text-amber-400 font-bold">★</span>
            {:else if item.is_dir}
              <span class="text-slate-600">▪</span>
            {:else}
              <span class="text-slate-700">▫</span>
            {/if}
          </td>

          <td class="px-3 py-1 flex items-center gap-2 truncate max-w-xs">
            <svelte:component
              this={getFileIcon(item)}
              class="w-4 h-4 flex-shrink-0 {item.is_dir ? 'text-sky-400' : item.name.endsWith('.rs') ? 'text-orange-400' : 'text-slate-400'}"
            />
            <span class="truncate {item.is_dir ? 'text-sky-300 font-semibold' : ''}">
              {item.name}
            </span>
          </td>

          <td class="w-24 px-3 py-1 text-right font-mono text-xs">
            {formatBytes(item.size)}
          </td>

          <td class="w-36 px-3 py-1 text-right text-xs text-slate-400">
            {formatDate(item.modified)}
          </td>

          <td class="w-28 px-3 py-1 text-center text-xs text-slate-500 hidden md:table-cell">
            {item.permissions}
          </td>
        </tr>
      {/each}
      {#if files.length === 0}
        <tr>
          <td colspan="5" class="px-4 py-8 text-center text-slate-500 italic">
            Mappen är tom
          </td>
        </tr>
      {/if}
    </tbody>
  </table>
</div>
