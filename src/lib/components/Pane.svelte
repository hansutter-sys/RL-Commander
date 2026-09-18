<script lang="ts">
  import { onMount } from "svelte";
  import type { DriveInfo, FileItem, PaneId, PaneState } from "../types";
  import { listDrives } from "../tauri";
  import { activePane, navigatePane, refreshPane, toggleSelection, showSftpModal } from "../stores/commander";
  import FileList from "./FileList.svelte";
  import { HardDrive, Server, RefreshCw, ArrowUp, FolderPlus } from "lucide-svelte";

  export let state: PaneState;
  export let paneId: PaneId;

  let drives: DriveInfo[] = [];

  onMount(async () => {
    drives = await listDrives();
    refreshPane(paneId);
  });

  function handleDriveChange(e: Event) {
    const target = e.target as HTMLSelectElement;
    navigatePane(paneId, target.value);
  }

  function handleSelectRow(index: number) {
    state.selectedIndex = index;
    activePane.set(paneId);
  }

  function handleOpen(item: FileItem) {
    if (item.is_dir) {
      navigatePane(paneId, item.path);
    }
  }

  function handleToggle(item: FileItem) {
    toggleSelection(paneId, item.path);
  }

  $: isActive = $activePane === paneId;
  $: selectedCount = state.selectedPaths.size;
  $: totalSize = state.files.reduce((acc, f) => acc + f.size, 0);
</script>

<div
  class="flex-1 flex flex-col h-full bg-slate-900 border rounded-lg overflow-hidden shadow-xl transition-all duration-150
    {isActive ? 'border-blue-500 ring-1 ring-blue-500/50' : 'border-slate-800 opacity-90'}"
  on:click={() => activePane.set(paneId)}
>
  <!-- Drive bar & Path Breadcrumb -->
  <div class="bg-slate-950 px-3 py-2 border-b border-slate-800 flex items-center justify-between gap-2">
    <div class="flex items-center gap-2 flex-1 min-w-0">
      <HardDrive class="w-4 h-4 text-blue-400 flex-shrink-0" />
      <select
        value={state.currentPath}
        on:change={handleDriveChange}
        class="bg-slate-900 text-xs font-mono text-slate-200 border border-slate-700 rounded px-2 py-1 outline-none focus:border-blue-500"
      >
        {#each drives as drive}
          <option value={drive.path}>{drive.name}</option>
        {/each}
        {#if state.isSftp}
          <option value={state.currentPath}>SFTP: {state.sftpConfig?.host}</option>
        {/if}
      </select>

      <span class="text-xs font-mono font-semibold text-slate-200 truncate flex-1 bg-slate-900 px-2 py-1 rounded border border-slate-800">
        {state.currentPath}
      </span>
    </div>

    <div class="flex items-center gap-1">
      <button
        on:click={() => showSftpModal.set(true)}
        title="Anslut SFTP"
        class="p-1.5 rounded bg-purple-950/50 hover:bg-purple-900 text-purple-300 border border-purple-800 text-xs flex items-center gap-1 font-semibold"
      >
        <Server class="w-3.5 h-3.5" />
        SFTP
      </button>

      <button
        on:click={() => refreshPane(paneId)}
        title="Uppdatera"
        class="p-1.5 rounded bg-slate-800 hover:bg-slate-700 text-slate-300 border border-slate-700"
      >
        <RefreshCw class="w-3.5 h-3.5 {state.loading ? 'animate-spin text-blue-400' : ''}" />
      </button>
    </div>
  </div>

  <!-- File Table -->
  <FileList
    files={state.files}
    selectedIndex={state.selectedIndex}
    selectedPaths={state.selectedPaths}
    {isActive}
    {paneId}
    onSelect={handleSelectRow}
    onOpen={handleOpen}
    onToggleSelection={handleToggle}
  />

  <!-- Status Bar Footer -->
  <div class="bg-slate-950 px-3 py-1.5 border-t border-slate-800 text-xs font-mono text-slate-400 flex items-center justify-between select-none">
    <div>
      {#if selectedCount > 0}
        <span class="text-amber-400 font-bold">{selectedCount} markerad(e)</span>
      {:else}
        <span>{state.files.length} objekt</span>
      {/if}
    </div>
    <div class="text-slate-500">
      {(totalSize / (1024 * 1024)).toFixed(1)} MB totalt
    </div>
  </div>
</div>
