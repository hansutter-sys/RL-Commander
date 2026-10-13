<script lang="ts">
  import { onMount } from "svelte";
  import type { DriveInfo, FileItem, PaneId, PaneState } from "../types";
  import { listDrives, copyItemsAsync, unzipArchive } from "../tauri";
  import { settings } from "../stores/settings";
  import {
    activePane,
    navigatePane,
    refreshPane,
    toggleSelection,
    selectRange,
    draggedItems,
    activeBookmarkDropdownPane,
    showSftpModal,
    showSearchModal,
    showViewerModal,
    showEditorModal,
    showDeleteModal,
    viewerFilePath,
    editorFilePath,
    openZipModal,
    leftPane,
    rightPane,
    bookmarks,
    addBookmark,
    removeBookmark,
  } from "../stores/commander";
  import FileList from "./FileList.svelte";
  import {
    HardDrive,
    Server,
    RefreshCw,
    Filter,
    Search,
    Upload,
    Bookmark as BookmarkIcon,
    Plus,
    Trash2,
    ChevronDown,
    Check,
    FolderPlus,
  } from "lucide-svelte";

  export let state: PaneState;
  export let paneId: PaneId;

  let drives: DriveInfo[] = [];
  let filterInput = "";
  let isDragOver = false;
  let dragEnterCounter = 0;
  let showBookmarkDropdown = false;
  let bookmarkCustomName = "";
  let isAddingCustomBookmark = false;

  $: if ($activeBookmarkDropdownPane === paneId) {
    showBookmarkDropdown = true;
  } else if ($activeBookmarkDropdownPane !== null && $activeBookmarkDropdownPane !== paneId) {
    showBookmarkDropdown = false;
  }

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
    } else {
      viewerFilePath.set(item.path);
      showViewerModal.set(true);
    }
  }

  function handleToggle(item: FileItem) {
    toggleSelection(paneId, item.path);
  }

  function handleView(item: FileItem) {
    if (!item.is_dir) {
      viewerFilePath.set(item.path);
      showViewerModal.set(true);
    }
  }

  function handleEdit(item: FileItem) {
    if (!item.is_dir) {
      editorFilePath.set(item.path);
      showEditorModal.set(true);
    }
  }

  async function handleCopy(item: FileItem, isMove = false) {
    const destPaneState = paneId === "left" ? $rightPane : $leftPane;
    let paths = Array.from(state.selectedPaths);
    if (paths.length === 0) {
      paths = [item.path];
    }
    try {
      await copyItemsAsync(paths, destPaneState.currentPath, isMove, state.sftpConfig);
      refreshPane("left");
      refreshPane("right");
    } catch (err: any) {
      alert(`Fel vid ${isMove ? "flytt" : "kopiering"}:\n${err?.toString()}`);
    }
  }

  function handleDelete(item: FileItem) {
    activePane.set(paneId);
    showDeleteModal.set(true);
  }

  function handleZip(item: FileItem) {
    let paths = Array.from(state.selectedPaths);
    if (paths.length === 0) {
      paths = [item.path];
    }
    openZipModal(paths);
  }

  async function handleUnzip(item: FileItem) {
    try {
      await unzipArchive(item.path, state.currentPath, state.sftpConfig);
      refreshPane("left");
      refreshPane("right");
    } catch (err: any) {
      alert(`Fel vid uppackning:\n${err?.toString()}`);
    }
  }

  function handleDragEnter(e: DragEvent) {
    e.preventDefault();
    dragEnterCounter++;
    // Don't show drop overlay if dragging over same pane without subfolder
    if ($draggedItems && $draggedItems.sourcePaneId === paneId) {
      return;
    }
    isDragOver = true;
  }

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    if (e.dataTransfer) {
      e.dataTransfer.dropEffect = "copy";
    }
  }

  function handleDragLeave(e: DragEvent) {
    dragEnterCounter--;
    if (dragEnterCounter <= 0) {
      dragEnterCounter = 0;
      isDragOver = false;
    }
  }

  async function handleDrop(e: DragEvent, targetDir?: string) {
    e.preventDefault();
    isDragOver = false;
    dragEnterCounter = 0;

    let dragData = $draggedItems;
    let paths: string[] = [];
    let sourcePaneId: PaneId = paneId === "left" ? "right" : "left";

    if (dragData && dragData.paths.length > 0) {
      paths = dragData.paths;
      sourcePaneId = dragData.sourcePaneId;
    } else {
      const rawData = e.dataTransfer?.getData("application/json") || e.dataTransfer?.getData("text/plain");
      if (rawData) {
        try {
          const parsed = JSON.parse(rawData);
          if (parsed.paths) {
            paths = parsed.paths;
            sourcePaneId = parsed.sourcePaneId;
          } else if (Array.isArray(parsed)) {
            paths = parsed;
          }
        } catch {}
      }
    }

    draggedItems.set(null);

    if (!paths || paths.length === 0) return;

    const destPath = targetDir || state.currentPath;

    // Prevent dropping to exact same directory in source pane
    if (sourcePaneId === paneId && destPath === state.currentPath) {
      return;
    }

    try {
      const sourceConfig = sourcePaneId === "left" ? $leftPane.sftpConfig : $rightPane.sftpConfig;
      await copyItemsAsync(paths, destPath, false, sourceConfig);
      refreshPane("left");
      refreshPane("right");
    } catch (err: any) {
      alert(`Kopieringsfel vid släpp: ${err?.toString()}`);
    }
  }

  function handleAddCurrentBookmark() {
    addBookmark(state.currentPath, bookmarkCustomName);
    bookmarkCustomName = "";
    isAddingCustomBookmark = false;
    showBookmarkDropdown = false;
    activeBookmarkDropdownPane.set(null);
  }

  function formatBytes(bytes: number): string {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + " " + sizes[i];
  }

  $: isActive = $activePane === paneId;
  $: selectedCount = state.selectedPaths.size;

  $: filteredFiles = filterInput.trim()
    ? state.files.filter((f) => f.name === ".." || f.name.toLowerCase().includes(filterInput.toLowerCase()))
    : state.files;

  $: totalSize = filteredFiles.reduce((acc, f) => acc + f.size, 0);

  // Active drive info calculation
  $: currentDrive = drives.find((d) => state.currentPath.startsWith(d.path)) || drives[0];
  $: driveTotal = currentDrive?.total_space || 0;
  $: driveAvailable = currentDrive?.available_space || 0;
  $: driveUsed = driveTotal > 0 ? driveTotal - driveAvailable : 0;
  $: driveUsedPercent = driveTotal > 0 ? Math.min(100, Math.round((driveUsed / driveTotal) * 100)) : 0;
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
  class="flex-1 flex flex-col h-full border rounded-lg overflow-hidden shadow-xl transition-all duration-150 relative
    {$settings.theme === 'light' ? 'bg-white' : 'bg-slate-900'}
    {isDragOver
      ? 'border-emerald-400 ring-4 ring-emerald-500/40 bg-emerald-950/20'
      : isActive
      ? 'border-blue-500 ring-1 ring-blue-500/50'
      : $settings.theme === 'light'
      ? 'border-slate-300 opacity-95'
      : 'border-slate-800 opacity-90'}"
  on:click={() => activePane.set(paneId)}
  on:dragenter={handleDragEnter}
  on:dragover={handleDragOver}
  on:dragleave={handleDragLeave}
  on:drop={(e) => handleDrop(e)}
>
  <!-- Drag over overlay visual indicator -->
  {#if isDragOver}
    <div
      class="absolute inset-0 bg-emerald-950/80 backdrop-blur-xs z-40 flex flex-col items-center justify-center border-2 border-dashed border-emerald-400 pointer-events-none animate-in fade-in duration-100"
    >
      <div
        class="bg-emerald-900/90 border border-emerald-500/60 p-4 rounded-xl shadow-2xl flex flex-col items-center gap-2 text-emerald-200 font-mono"
      >
        <Upload class="w-8 h-8 text-emerald-400 animate-bounce" />
        <span class="font-bold text-sm">Släpp för att kopiera till {paneId === 'left' ? 'vänster' : 'höger'} panel</span>
        <span class="text-xs text-emerald-300 max-w-sm truncate">{state.currentPath}</span>
      </div>
    </div>
  {/if}

  <!-- Pane Header: Drive bar & Path Breadcrumb -->
  <div
    class="px-3 py-2 border-b flex flex-col gap-1.5 z-10 transition-colors
      {$settings.theme === 'light' ? 'bg-slate-50 border-slate-200' : 'bg-slate-950 border-slate-800'}"
  >
    <!-- Drive Quick Button Bar (Setting dependent) -->
    {#if $settings.showDriveButtons && drives.length > 0}
      <div
        class="flex items-center gap-1 overflow-x-auto pb-1 text-xs font-mono scrollbar-none border-b
          {$settings.theme === 'light' ? 'border-slate-200' : 'border-slate-800/60'}"
      >
        <span class="text-[10px] uppercase font-bold mr-1 {$settings.theme === 'light' ? 'text-slate-500' : 'text-slate-500'}">Diskar:</span>
        {#each drives as d}
          <button
            on:click={() => navigatePane(paneId, d.path)}
            class="px-2 py-0.5 rounded border text-xs font-semibold flex items-center gap-1 transition-all whitespace-nowrap
              {state.currentPath.startsWith(d.path)
                ? 'bg-blue-600 border-blue-500 text-white shadow-sm shadow-blue-500/30'
                : $settings.theme === 'light'
                ? 'bg-white hover:bg-slate-100 border-slate-300 text-slate-700'
                : 'bg-slate-900 hover:bg-slate-800 border-slate-700 text-slate-300'}"
          >
            <HardDrive class="w-3 h-3" />
            <span>{d.name}</span>
          </button>
        {/each}
      </div>
    {/if}

    <!-- Drive selector dropdown & Path display -->
    <div class="flex items-center justify-between gap-2">
      <div class="flex items-center gap-2 flex-1 min-w-0">
        <HardDrive class="w-4 h-4 text-blue-500 flex-shrink-0" />
        <select
          value={state.currentPath}
          on:change={handleDriveChange}
          class="text-xs font-mono border rounded px-2 py-1 outline-none focus:border-blue-500
            {$settings.theme === 'light' ? 'bg-white text-slate-800 border-slate-300' : 'bg-slate-900 text-slate-200 border-slate-700'}"
        >
          {#each drives as drive}
            <option value={drive.path}>{drive.name}</option>
          {/each}
          {#if state.isSftp}
            <option value={state.currentPath}>SFTP: {state.sftpConfig?.host}</option>
          {/if}
        </select>

        <span
          class="text-xs font-mono font-semibold truncate flex-1 px-2 py-1 rounded border
            {$settings.theme === 'light' ? 'bg-white text-slate-900 border-slate-300 shadow-2xs' : 'bg-slate-900 text-slate-200 border-slate-800'}"
        >
          {state.currentPath}
        </span>
      </div>

      <div class="flex items-center gap-1">
        <button
          on:click={() => showSearchModal.set(true)}
          title="Sök i undermappar (Alt+F7)"
          class="p-1.5 rounded text-xs flex items-center gap-1 font-semibold transition-colors
            {$settings.theme === 'light'
              ? 'bg-indigo-50 hover:bg-indigo-100 text-indigo-700 border border-indigo-200'
              : 'bg-indigo-950/60 hover:bg-indigo-900 text-indigo-300 border border-indigo-800/80'}"
        >
          <Search class="w-3.5 h-3.5" />
          <span class="hidden sm:inline">Sök</span>
        </button>

        <button
          on:click={() => showSftpModal.set(true)}
          title="Anslut SFTP"
          class="p-1.5 rounded text-xs flex items-center gap-1 font-semibold transition-colors
            {$settings.theme === 'light'
              ? 'bg-purple-50 hover:bg-purple-100 text-purple-700 border border-purple-200'
              : 'bg-purple-950/50 hover:bg-purple-900 text-purple-300 border border-purple-800'}"
        >
          <Server class="w-3.5 h-3.5" />
          <span>SFTP</span>
        </button>

        <button
          on:click={() => refreshPane(paneId)}
          title="Uppdatera"
          class="p-1.5 rounded border transition-colors
            {$settings.theme === 'light'
              ? 'bg-white hover:bg-slate-100 text-slate-700 border-slate-300'
              : 'bg-slate-800 hover:bg-slate-700 text-slate-300 border-slate-700'}"
        >
          <RefreshCw class="w-3.5 h-3.5 {state.loading ? 'animate-spin text-blue-500' : ''}" />
        </button>
      </div>
    </div>

    <!-- Disk Usage / Free Space Line (Setting dependent) -->
    {#if $settings.showDiskSpace && currentDrive && driveTotal > 0}
      <div
        class="border rounded px-2.5 py-1 text-[11px] font-mono flex flex-col sm:flex-row sm:items-center justify-between gap-1 transition-colors
          {$settings.theme === 'light' ? 'bg-white border-slate-200 text-slate-700' : 'bg-slate-900/80 border-slate-800/80 text-slate-300'}"
      >
        <div class="flex items-center gap-2">
          <span class="font-semibold {$settings.theme === 'light' ? 'text-slate-600' : 'text-slate-400'}">{currentDrive.name}:</span>
          <span>Använt: <strong class="text-blue-500">{formatBytes(driveUsed)}</strong></span>
          <span class="{$settings.theme === 'light' ? 'text-slate-300' : 'text-slate-600'}">|</span>
          <span>Ledigt: <strong class="text-emerald-500">{formatBytes(driveAvailable)}</strong> av {formatBytes(driveTotal)}</span>
        </div>

        <div class="flex items-center gap-2 w-full sm:w-36">
          <div class="flex-1 rounded-full h-2 overflow-hidden border {$settings.theme === 'light' ? 'bg-slate-100 border-slate-200' : 'bg-slate-950 border-slate-800'}">
            <div
              class="h-full transition-all duration-300 rounded-full
                {driveUsedPercent >= 90 ? 'bg-rose-500' : driveUsedPercent >= 75 ? 'bg-amber-500' : 'bg-emerald-500'}"
              style="width: {driveUsedPercent}%;"
            ></div>
          </div>
          <span class="text-[10px] font-bold w-8 text-right {$settings.theme === 'light' ? 'text-slate-600' : 'text-slate-400'}">
            {driveUsedPercent}%
          </span>
        </div>
      </div>
    {/if}

    <!-- Bookmarks Dropdown & Filter Bar -->
    <div class="flex items-center justify-between gap-2 pt-0.5">
      <!-- Bookmarks Dropdown Menu -->
      <div class="relative">
        <button
          on:click={() => {
            showBookmarkDropdown = !showBookmarkDropdown;
            if (!showBookmarkDropdown) activeBookmarkDropdownPane.set(null);
          }}
          title="Genvägar och sparade platser (Ctrl+D)"
          class="px-2.5 py-1 rounded border text-xs font-mono flex items-center gap-1.5 transition-colors
            {$settings.theme === 'light'
              ? 'bg-white hover:bg-slate-100 text-slate-700 border-slate-300'
              : 'bg-slate-900 hover:bg-slate-800 text-slate-300 border-slate-700'}"
        >
          <BookmarkIcon class="w-3.5 h-3.5 text-amber-500" />
          <span>Genvägar</span>
          <ChevronDown class="w-3 h-3 text-slate-400" />
        </button>

        {#if showBookmarkDropdown}
          <!-- svelte-ignore a11y-click-events-have-key-events -->
          <div
            class="absolute left-0 top-full mt-1 z-50 w-72 border rounded-lg shadow-2xl py-1.5 text-xs font-mono backdrop-blur-md transition-all
              {$settings.theme === 'light' ? 'bg-white/95 border-slate-300 text-slate-800' : 'bg-slate-900/95 border-slate-700 text-slate-200'}"
            on:click|stopPropagation
          >
            <div class="px-3 py-1.5 border-b flex items-center justify-between text-[11px] font-bold
              {$settings.theme === 'light' ? 'border-slate-200 text-slate-600' : 'border-slate-800 text-slate-400'}">
              <span>Sparade genvägar</span>
              {#if !isAddingCustomBookmark}
                <button
                  on:click={() => {
                    isAddingCustomBookmark = true;
                    bookmarkCustomName = state.currentPath.split("/").filter(Boolean).pop() || state.currentPath;
                  }}
                  class="text-blue-500 hover:text-blue-600 flex items-center gap-1 hover:underline"
                >
                  <Plus class="w-3 h-3" />
                  <span>Spara denna</span>
                </button>
              {/if}
            </div>

            <!-- Custom Name Input Form -->
            {#if isAddingCustomBookmark}
              <div class="p-2 border-b {$settings.theme === 'light' ? 'border-slate-200 bg-slate-50' : 'border-slate-800 bg-slate-950/60'}">
                <div class="text-[10px] text-slate-500 mb-1">Namn på genväg:</div>
                <div class="flex items-center gap-1.5">
                  <input
                    type="text"
                    bind:value={bookmarkCustomName}
                    placeholder="Mina filer..."
                    class="flex-1 px-2 py-1 text-xs rounded border outline-none
                      {$settings.theme === 'light' ? 'bg-white border-slate-300 text-slate-800' : 'bg-slate-900 border-slate-700 text-slate-100'}"
                    on:keydown={(e) => {
                      if (e.key === "Enter") handleAddCurrentBookmark();
                      if (e.key === "Escape") isAddingCustomBookmark = false;
                    }}
                  />
                  <button
                    on:click={handleAddCurrentBookmark}
                    class="bg-blue-600 hover:bg-blue-500 text-white px-2 py-1 rounded font-semibold text-xs flex items-center gap-1"
                  >
                    <Check class="w-3 h-3" />
                    <span>Spara</span>
                  </button>
                  <button
                    on:click={() => (isAddingCustomBookmark = false)}
                    class="text-slate-400 hover:text-slate-600 px-1 py-1 text-xs"
                  >
                    ✕
                  </button>
                </div>
              </div>
            {/if}

            <div class="max-h-60 overflow-y-auto py-1">
              {#each $bookmarks as bm}
                <div
                  class="flex items-center justify-between px-2.5 py-1.5 group transition-colors
                    {$settings.theme === 'light' ? 'hover:bg-slate-100' : 'hover:bg-slate-800'}"
                >
                  <button
                    on:click={() => {
                      navigatePane(paneId, bm.path);
                      showBookmarkDropdown = false;
                      activeBookmarkDropdownPane.set(null);
                    }}
                    class="flex-1 text-left truncate flex items-center gap-1.5
                      {$settings.theme === 'light' ? 'text-slate-800 group-hover:text-blue-600' : 'text-slate-200 group-hover:text-white'}"
                  >
                    <BookmarkIcon class="w-3 h-3 text-amber-500 flex-shrink-0" />
                    <span class="font-semibold truncate">{bm.name}</span>
                    <span class="text-[10px] text-slate-400 truncate font-mono">({bm.path})</span>
                  </button>

                  <button
                    on:click={() => removeBookmark(bm.path)}
                    title="Ta bort genväg"
                    class="opacity-0 group-hover:opacity-100 p-1 text-slate-400 hover:text-red-500 transition-opacity ml-1"
                  >
                    <Trash2 class="w-3 h-3" />
                  </button>
                </div>
              {/each}
              {#if $bookmarks.length === 0}
                <div class="px-3 py-3 text-center text-slate-400 italic">
                  Inga sparade genvägar än
                </div>
              {/if}
            </div>
          </div>
        {/if}
      </div>

      <!-- Live Quick Filter -->
      <div class="relative w-36 sm:w-44 flex-shrink-0">
        <input
          type="text"
          bind:value={filterInput}
          placeholder="Filter..."
          class="w-full border text-[11px] font-mono pl-6 pr-2 py-0.5 rounded outline-none focus:border-blue-500
            {$settings.theme === 'light' ? 'bg-white border-slate-300 text-slate-800' : 'bg-slate-900 border-slate-700 text-slate-200'}"
        />
        <Filter class="w-3 h-3 text-slate-400 absolute left-2 top-1.5" />
      </div>
    </div>
  </div>

  <!-- File Table -->
  <FileList
    files={filteredFiles}
    selectedIndex={state.selectedIndex}
    selectedPaths={state.selectedPaths}
    {isActive}
    {paneId}
    onSelect={handleSelectRow}
    onOpen={handleOpen}
    onToggleSelection={handleToggle}
    onRangeSelect={(from, to) => selectRange(paneId, from, to)}
    onDropToPath={(e, path) => handleDrop(e, path)}
    onView={handleView}
    onEdit={handleEdit}
    onCopy={(item) => handleCopy(item, false)}
    onMove={(item) => handleCopy(item, true)}
    onDelete={handleDelete}
    onZip={handleZip}
    onUnzip={handleUnzip}
  />

  <!-- Status Bar Footer -->
  <div
    class="px-3 py-1.5 border-t text-xs font-mono flex items-center justify-between select-none z-10 transition-colors
      {$settings.theme === 'light' ? 'bg-slate-100 border-slate-200 text-slate-600' : 'bg-slate-950 border-slate-800 text-slate-400'}"
  >
    <div>
      {#if selectedCount > 0}
        <span class="text-amber-500 font-bold">{selectedCount} markerad(e)</span>
      {:else if filterInput.trim()}
        <span class="text-blue-500 font-medium">{filteredFiles.length} av {state.files.length} matchar filter</span>
      {:else}
        <span>{state.files.length} objekt</span>
      {/if}
    </div>
    <div class="text-slate-400 text-[11px]">
      {(totalSize / (1024 * 1024)).toFixed(1)} MB totalt
    </div>
  </div>
</div>
