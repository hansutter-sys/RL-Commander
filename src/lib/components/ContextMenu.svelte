<script lang="ts">
  import { onMount } from "svelte";
  import type { FileItem, PaneId } from "../types";
  import { Archive, FolderArchive, Eye, Edit3, Copy, Move, Trash2 } from "lucide-svelte";

  export let x: number = 0;
  export let y: number = 0;
  export let visible: boolean = false;
  export let item: FileItem | null = null;
  export let paneId: PaneId;

  export let onClose: () => void;
  export let onZip: (item: FileItem) => void;
  export let onUnzip: (item: FileItem) => void;
  export let onView: (item: FileItem) => void;
  export let onEdit: (item: FileItem) => void;
  export let onCopy: (item: FileItem) => void;
  export let onMove: (item: FileItem) => void;
  export let onDelete: (item: FileItem) => void;

  let menuRef: HTMLDivElement;
  let adjustedX = x;
  let adjustedY = y;

  $: if (visible) {
    adjustedX = Math.min(x, typeof window !== "undefined" ? window.innerWidth - 220 : x);
    adjustedY = Math.min(y, typeof window !== "undefined" ? window.innerHeight - 260 : y);
  }

  $: isZipFile = item ? item.name.toLowerCase().endsWith(".zip") : false;

  function handleWindowClick(e: MouseEvent) {
    if (visible && menuRef && !menuRef.contains(e.target as Node)) {
      onClose();
    }
  }

  function handleKeyDown(e: KeyboardEvent) {
    if (visible && e.key === "Escape") {
      onClose();
    }
  }
</script>

<svelte:window on:click={handleWindowClick} on:keydown={handleKeyDown} />

{#if visible && item && item.name !== ".."}
  <div
    bind:this={menuRef}
    style="left: {adjustedX}px; top: {adjustedY}px;"
    class="fixed z-50 w-56 bg-slate-900 border border-slate-700 shadow-2xl rounded-lg py-1.5 text-xs font-mono text-slate-200 backdrop-blur-md select-none animate-in fade-in zoom-in-95 duration-100"
  >
    <!-- Item Header -->
    <div class="px-3 py-1 border-b border-slate-800 text-[11px] text-slate-400 font-semibold truncate flex items-center justify-between">
      <span class="truncate">{item.name}</span>
      <span class="text-[10px] text-blue-400 uppercase font-bold ml-1">{paneId}</span>
    </div>

    <div class="py-1">
      <!-- Zip option -->
      <button
        on:click={() => { item && onZip(item); onClose(); }}
        class="w-full px-3 py-1.5 text-left hover:bg-blue-600 hover:text-white flex items-center justify-between transition-colors group"
      >
        <div class="flex items-center gap-2">
          <Archive class="w-4 h-4 text-amber-400 group-hover:text-white" />
          <span>Komprimera till ZIP...</span>
        </div>
      </button>

      <!-- Unzip option (Only if zip file) -->
      {#if isZipFile}
        <button
          on:click={() => { item && onUnzip(item); onClose(); }}
          class="w-full px-3 py-1.5 text-left hover:bg-blue-600 hover:text-white flex items-center justify-between transition-colors group bg-amber-950/30"
        >
          <div class="flex items-center gap-2">
            <FolderArchive class="w-4 h-4 text-amber-400 group-hover:text-white" />
            <span class="text-amber-300 group-hover:text-white font-semibold">Packa upp ZIP här</span>
          </div>
        </button>
      {/if}

      <div class="my-1 border-t border-slate-800"></div>

      <!-- View -->
      {#if !item.is_dir}
        <button
          on:click={() => { item && onView(item); onClose(); }}
          class="w-full px-3 py-1.5 text-left hover:bg-blue-600 hover:text-white flex items-center justify-between transition-colors group"
        >
          <div class="flex items-center gap-2">
            <Eye class="w-4 h-4 text-sky-400 group-hover:text-white" />
            <span>Visa</span>
          </div>
          <span class="text-[10px] text-slate-400 group-hover:text-slate-200">F3</span>
        </button>

        <!-- Edit -->
        <button
          on:click={() => { item && onEdit(item); onClose(); }}
          class="w-full px-3 py-1.5 text-left hover:bg-blue-600 hover:text-white flex items-center justify-between transition-colors group"
        >
          <div class="flex items-center gap-2">
            <Edit3 class="w-4 h-4 text-emerald-400 group-hover:text-white" />
            <span>Redigera</span>
          </div>
          <span class="text-[10px] text-slate-400 group-hover:text-slate-200">F4</span>
        </button>
      {/if}

      <!-- Copy -->
      <button
        on:click={() => { item && onCopy(item); onClose(); }}
        class="w-full px-3 py-1.5 text-left hover:bg-blue-600 hover:text-white flex items-center justify-between transition-colors group"
      >
        <div class="flex items-center gap-2">
          <Copy class="w-4 h-4 text-indigo-400 group-hover:text-white" />
          <span>Kopiera till motsatt panel</span>
        </div>
        <span class="text-[10px] text-slate-400 group-hover:text-slate-200">F5</span>
      </button>

      <!-- Move -->
      <button
        on:click={() => { item && onMove(item); onClose(); }}
        class="w-full px-3 py-1.5 text-left hover:bg-blue-600 hover:text-white flex items-center justify-between transition-colors group"
      >
        <div class="flex items-center gap-2">
          <Move class="w-4 h-4 text-purple-400 group-hover:text-white" />
          <span>Flytta till motsatt panel</span>
        </div>
        <span class="text-[10px] text-slate-400 group-hover:text-slate-200">F6</span>
      </button>

      <div class="my-1 border-t border-slate-800"></div>

      <!-- Delete -->
      <button
        on:click={() => { item && onDelete(item); onClose(); }}
        class="w-full px-3 py-1.5 text-left hover:bg-red-600 hover:text-white text-red-400 flex items-center justify-between transition-colors group"
      >
        <div class="flex items-center gap-2">
          <Trash2 class="w-4 h-4 text-red-400 group-hover:text-white" />
          <span>Ta bort</span>
        </div>
        <span class="text-[10px] text-red-400/80 group-hover:text-slate-200">F8</span>
      </button>
    </div>
  </div>
{/if}
