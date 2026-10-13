<script lang="ts">
  import { onMount } from "svelte";
  import Pane from "./lib/components/Pane.svelte";
  import Splitter from "./lib/components/Splitter.svelte";
  import Toolbar from "./lib/components/Toolbar.svelte";
  import FileViewerModal from "./lib/components/FileViewerModal.svelte";
  import FileEditorModal from "./lib/components/FileEditorModal.svelte";
  import SearchModal from "./lib/components/SearchModal.svelte";
  import CreateDirModal from "./lib/components/CreateDirModal.svelte";
  import DeleteConfirmModal from "./lib/components/DeleteConfirmModal.svelte";
  import SftpModal from "./lib/components/SftpModal.svelte";
  import SettingsModal from "./lib/components/SettingsModal.svelte";
  import ZipModal from "./lib/components/ZipModal.svelte";
  import {
    leftPane,
    rightPane,
    activePane,
    activePaneState,
    inactivePaneState,
    toggleActivePane,
    navigatePane,
    refreshPane,
    toggleSelection,
    selectAll,
    invertSelection,
    clearSelection,
    toggleBookmarkDropdown,
    toastMessage,
    showViewerModal,
    showEditorModal,
    showSearchModal,
    showCreateDirModal,
    showDeleteModal,
    showSftpModal,
    showSettingsModal,
    showZipModal,
    viewerFilePath,
    editorFilePath,
    activeTransfer,
  } from "./lib/stores/commander";
  import { settings } from "./lib/stores/settings";
  import { copyItemsAsync, deleteItems } from "./lib/tauri";
  import type { TransferProgressEvent } from "./lib/types";
  import {
    Cpu,
    ArrowLeftRight,
    CheckCircle2,
    RefreshCw,
    Server,
    Search,
    Settings,
    Sun,
    Moon,
  } from "lucide-svelte";

  import { initSftpProfiles } from "./lib/stores/sftpProfiles";

  let isTransferring = false;
  let transferNotification: string | null = null;

  // Splitter resizable width state
  let mainContainerRef: HTMLElement;
  let splitRatio = 50; // percentage for left pane
  let isDraggingSplitter = false;

  onMount(() => {
    initSftpProfiles();
    // Load split ratio preference
    if (typeof localStorage !== "undefined") {
      const savedRatio = localStorage.getItem("rl_commander_split_ratio");
      if (savedRatio) {
        const parsed = parseFloat(savedRatio);
        if (!isNaN(parsed) && parsed >= 15 && parsed <= 85) {
          splitRatio = parsed;
        }
      }
    }

    // Listen for Tauri transfer-progress events if in Tauri env
    if (typeof window !== "undefined" && "__TAURI_INTERNALS__" in window) {
      import("@tauri-apps/api/event").then(({ listen }) => {
        listen<TransferProgressEvent>("transfer-progress", (event) => {
          activeTransfer.set(event.payload);
          if (event.payload.is_finished) {
            refreshPane("left");
            refreshPane("right");
            setTimeout(() => activeTransfer.set(null), 3000);
          }
        });
      });
    }
  });

  // Theme synchronization with HTML root element
  $: if (typeof document !== "undefined") {
    if ($settings.theme === "light") {
      document.documentElement.classList.remove("dark");
      document.documentElement.classList.add("light");
    } else {
      document.documentElement.classList.remove("light");
      document.documentElement.classList.add("dark");
    }
  }

  // Splitter resizing handlers
  function handleStartResize(e: CustomEvent<MouseEvent>) {
    isDraggingSplitter = true;
    window.addEventListener("mousemove", handleSplitterMouseMove);
    window.addEventListener("mouseup", handleSplitterMouseUp);
  }

  function handleSplitterMouseMove(e: MouseEvent) {
    if (!isDraggingSplitter || !mainContainerRef) return;
    const rect = mainContainerRef.getBoundingClientRect();
    const newPercent = ((e.clientX - rect.left) / rect.width) * 100;
    splitRatio = Math.max(15, Math.min(85, newPercent));
  }

  function handleSplitterMouseUp() {
    if (isDraggingSplitter) {
      isDraggingSplitter = false;
      window.removeEventListener("mousemove", handleSplitterMouseMove);
      window.removeEventListener("mouseup", handleSplitterMouseUp);
      if (typeof localStorage !== "undefined") {
        localStorage.setItem("rl_commander_split_ratio", splitRatio.toFixed(1));
      }
    }
  }

  function handleResetResize() {
    splitRatio = 50;
    if (typeof localStorage !== "undefined") {
      localStorage.setItem("rl_commander_split_ratio", "50");
    }
  }

  function handleKeyDown(e: KeyboardEvent) {
    // Disable global hotkeys if any modal is open
    let modalsOpen = false;
    showViewerModal.subscribe((v) => (modalsOpen = modalsOpen || v))();
    showEditorModal.subscribe((v) => (modalsOpen = modalsOpen || v))();
    showSearchModal.subscribe((v) => (modalsOpen = modalsOpen || v))();
    showCreateDirModal.subscribe((v) => (modalsOpen = modalsOpen || v))();
    showDeleteModal.subscribe((v) => (modalsOpen = modalsOpen || v))();
    showSftpModal.subscribe((v) => (modalsOpen = modalsOpen || v))();
    showSettingsModal.subscribe((v) => (modalsOpen = modalsOpen || v))();
    showZipModal.subscribe((v) => (modalsOpen = modalsOpen || v))();

    if (modalsOpen) {
      if (e.key === "Escape") {
        showViewerModal.set(false);
        showCreateDirModal.set(false);
        showDeleteModal.set(false);
        showSftpModal.set(false);
        showSearchModal.set(false);
        showSettingsModal.set(false);
        showZipModal.set(false);
      }
      return;
    }

    const currentPaneId = $activePane;
    const currentStore = currentPaneId === "left" ? leftPane : rightPane;
    const state = $activePaneState;

    // Alt+F7 Search
    if (e.altKey && e.key === "F7") {
      e.preventDefault();
      showSearchModal.set(true);
      return;
    }

    // Ctrl+A: Select all files in active pane
    if ((e.ctrlKey || e.metaKey) && (e.key === "a" || e.key === "A")) {
      e.preventDefault();
      selectAll(currentPaneId);
      return;
    }

    // Ctrl+D: Directory Bookmarks / Genvägar
    if ((e.ctrlKey || e.metaKey) && (e.key === "d" || e.key === "D")) {
      e.preventDefault();
      toggleBookmarkDropdown(currentPaneId);
      return;
    }

    // Total Commander '*': Invert selection
    if (e.key === "*" && !e.ctrlKey && !e.altKey && !e.metaKey) {
      e.preventDefault();
      invertSelection(currentPaneId);
      return;
    }

    if (e.key === "Escape") {
      e.preventDefault();
      clearSelection(currentPaneId);
      return;
    }

    if (e.key === "Tab") {
      e.preventDefault();
      toggleActivePane();
      return;
    }

    if (e.key === "ArrowDown") {
      e.preventDefault();
      if (state.files.length > 0) {
        const nextIdx = Math.min(state.selectedIndex + 1, state.files.length - 1);
        currentStore.update((s) => ({ ...s, selectedIndex: nextIdx }));
      }
      return;
    }

    if (e.key === "ArrowUp") {
      e.preventDefault();
      if (state.files.length > 0) {
        const nextIdx = Math.max(state.selectedIndex - 1, 0);
        currentStore.update((s) => ({ ...s, selectedIndex: nextIdx }));
      }
      return;
    }

    if (e.key === " " || e.key === "Insert") {
      e.preventDefault();
      if (state.files.length > 0 && state.selectedIndex < state.files.length) {
        const item = state.files[state.selectedIndex];
        if (item.name !== "..") {
          toggleSelection(currentPaneId, item.path);
        }
        // Advance cursor
        const nextIdx = Math.min(state.selectedIndex + 1, state.files.length - 1);
        currentStore.update((s) => ({ ...s, selectedIndex: nextIdx }));
      }
      return;
    }

    if (e.key === "Enter") {
      e.preventDefault();
      if (state.files.length > 0 && state.selectedIndex < state.files.length) {
        const item = state.files[state.selectedIndex];
        if (item.is_dir) {
          navigatePane(currentPaneId, item.path);
        } else {
          // View file on enter if file
          viewerFilePath.set(item.path);
          showViewerModal.set(true);
        }
      }
      return;
    }

    if (e.key === "Backspace") {
      e.preventDefault();
      const parentDir = state.files.find((f) => f.name === "..");
      if (parentDir) {
        navigatePane(currentPaneId, parentDir.path);
      }
      return;
    }

    // Function keys (F3 - F8)
    if (e.key === "F3") {
      e.preventDefault();
      triggerView();
      return;
    }

    if (e.key === "F4") {
      e.preventDefault();
      triggerEdit();
      return;
    }

    if (e.key === "F5") {
      e.preventDefault();
      triggerCopy();
      return;
    }

    if (e.key === "F6") {
      e.preventDefault();
      triggerMove();
      return;
    }

    if (e.key === "F7") {
      e.preventDefault();
      showCreateDirModal.set(true);
      return;
    }

    if (e.key === "F8") {
      e.preventDefault();
      triggerDelete();
      return;
    }
  }

  function triggerView() {
    const state = $activePaneState;
    if (state.files.length > 0 && state.selectedIndex < state.files.length) {
      const item = state.files[state.selectedIndex];
      if (!item.is_dir) {
        viewerFilePath.set(item.path);
        showViewerModal.set(true);
      }
    }
  }

  function triggerEdit() {
    const state = $activePaneState;
    if (state.files.length > 0 && state.selectedIndex < state.files.length) {
      const item = state.files[state.selectedIndex];
      if (!item.is_dir) {
        editorFilePath.set(item.path);
        showEditorModal.set(true);
      }
    }
  }

  async function triggerCopy(isMove = false) {
    const srcState = $activePaneState;
    const destState = $inactivePaneState;

    let pathsToCopy = Array.from(srcState.selectedPaths);
    if (pathsToCopy.length === 0 && srcState.files[srcState.selectedIndex]) {
      const current = srcState.files[srcState.selectedIndex];
      if (current.name !== "..") {
        pathsToCopy = [current.path];
      }
    }

    if (pathsToCopy.length === 0) return;

    try {
      const actionName = isMove ? "Flyttar" : "Kopierar";
      transferNotification = `${actionName} ${pathsToCopy.length} objekt till ${destState.currentPath}...`;
      isTransferring = true;

      await copyItemsAsync(pathsToCopy, destState.currentPath, isMove, srcState.sftpConfig);

      setTimeout(() => {
        refreshPane("left");
        refreshPane("right");
        isTransferring = false;
        transferNotification = `${actionName} slutförd!`;
        setTimeout(() => (transferNotification = null), 3000);
      }, 1200);
    } catch (err: any) {
      alert(`Kopieringsfel: ${err?.toString()}`);
      isTransferring = false;
    }
  }

  function triggerMove() {
    triggerCopy(true);
  }

  async function triggerDelete() {
    if ($settings.confirmOnDelete) {
      showDeleteModal.set(true);
    } else {
      const srcState = $activePaneState;
      let itemsToDelete = Array.from(srcState.selectedPaths).map((p) => {
        const fileObj = srcState.files.find((f) => f.path === p);
        return [p, fileObj ? fileObj.is_dir : false] as [string, boolean];
      });

      if (itemsToDelete.length === 0 && srcState.files[srcState.selectedIndex]) {
        const current = srcState.files[srcState.selectedIndex];
        if (current.name !== "..") {
          itemsToDelete = [[current.path, current.is_dir]];
        }
      }

      if (itemsToDelete.length === 0) return;

      try {
        await deleteItems(itemsToDelete, srcState.sftpConfig);
        refreshPane($activePane);
      } catch (e: any) {
        alert(`Fel vid radering:\n${e?.toString()}`);
      }
    }
  }
</script>

<svelte:window on:keydown={handleKeyDown} />

<div
  class="h-screen w-screen flex flex-col overflow-hidden select-none font-sans transition-colors duration-150
    {$settings.theme === 'light' ? 'bg-slate-100 text-slate-800' : 'bg-slate-950 text-slate-100'}"
>
  <!-- Top Application Bar -->
  <header
    class="px-4 py-2 flex items-center justify-between shadow-md border-b transition-colors
      {$settings.theme === 'light' ? 'bg-white border-slate-300' : 'bg-slate-900/90 border-slate-800'}"
  >
    <div class="flex items-center gap-3">
      <div
        class="w-8 h-8 rounded-lg bg-gradient-to-tr from-blue-600 via-indigo-500 to-purple-600 flex items-center justify-center shadow-lg shadow-blue-500/20"
      >
        <Cpu class="w-5 h-5 text-white" />
      </div>
      <div>
        <h1
          class="font-black text-base tracking-wider bg-gradient-to-r from-blue-500 via-indigo-500 to-purple-500 bg-clip-text text-transparent"
        >
          RL COMMANDER <span class="text-xs font-mono font-normal opacity-70">v1.2</span>
        </h1>
        <p class="text-[10px] font-mono opacity-60 -mt-0.5">High Performance Dual-Pane & SFTP Engine</p>
      </div>
    </div>

    <!-- Active Pane Status Indicator -->
    <div
      class="hidden sm:flex items-center gap-3 px-3 py-1.5 rounded-lg border text-xs font-mono transition-colors
        {$settings.theme === 'light' ? 'bg-slate-100 border-slate-300' : 'bg-slate-950 border-slate-800'}"
    >
      <div class="flex items-center gap-1.5">
        <span class="w-2 h-2 rounded-full bg-emerald-500 animate-pulse"></span>
        <span class="opacity-70">Aktiv:</span>
        <span class="font-bold text-blue-500 uppercase">{$activePane}</span>
      </div>
      <div class="h-3 w-px {$settings.theme === 'light' ? 'bg-slate-300' : 'bg-slate-800'}"></div>
      <button
        on:click={toggleActivePane}
        class="flex items-center gap-1 transition-colors px-1.5 py-0.5 rounded
          {$settings.theme === 'light' ? 'hover:bg-slate-200 text-slate-700' : 'hover:bg-slate-900 text-slate-400 hover:text-white'}"
      >
        <ArrowLeftRight class="w-3.5 h-3.5 text-indigo-500" />
        <span>Växla (Tab)</span>
      </button>
    </div>

    <div class="flex items-center gap-2">
      <!-- Search Button -->
      <button
        on:click={() => showSearchModal.set(true)}
        class="px-2.5 py-1.5 rounded-md text-xs font-mono font-semibold flex items-center gap-1.5 transition-all shadow-2xs
          {$settings.theme === 'light'
            ? 'bg-indigo-50 hover:bg-indigo-100 border border-indigo-200 text-indigo-700'
            : 'bg-indigo-950/60 hover:bg-indigo-900 border border-indigo-800/80 text-indigo-300'}"
      >
        <Search class="w-3.5 h-3.5 text-indigo-500" />
        <span>Sök (Alt+F7)</span>
      </button>

      <!-- SFTP Manager Button -->
      <button
        on:click={() => showSftpModal.set(true)}
        class="px-2.5 py-1.5 rounded-md text-xs font-mono font-semibold flex items-center gap-1.5 transition-all shadow-2xs
          {$settings.theme === 'light'
            ? 'bg-purple-50 hover:bg-purple-100 border border-purple-200 text-purple-700'
            : 'bg-purple-950/60 hover:bg-purple-900 border border-purple-800/80 text-purple-300'}"
      >
        <Server class="w-3.5 h-3.5 text-purple-500" />
        <span>SFTP</span>
      </button>

      <!-- Quick Theme Switch Button -->
      <button
        on:click={() => settings.update((s) => ({ ...s, theme: s.theme === "light" ? "dark" : "light" }))}
        class="border px-2.5 py-1.5 rounded-md text-xs font-mono font-semibold flex items-center gap-1.5 transition-all shadow-2xs
          {$settings.theme === 'light'
            ? 'bg-amber-50 hover:bg-amber-100 border-amber-200 text-amber-800'
            : 'bg-slate-800 hover:bg-slate-700 border-slate-700 text-slate-200'}"
        title="Växla mellan mörkt och ljust läge"
      >
        {#if $settings.theme === "light"}
          <Sun class="w-3.5 h-3.5 text-amber-600" />
          <span class="hidden md:inline">Ljust</span>
        {:else}
          <Moon class="w-3.5 h-3.5 text-indigo-400" />
          <span class="hidden md:inline">Mörkt</span>
        {/if}
      </button>

      <!-- Settings Button -->
      <button
        on:click={() => showSettingsModal.set(true)}
        class="border px-2.5 py-1.5 rounded-md text-xs font-mono font-semibold flex items-center gap-1.5 transition-all shadow-2xs
          {$settings.theme === 'light'
            ? 'bg-white hover:bg-slate-100 border-slate-300 text-slate-700'
            : 'bg-slate-800 hover:bg-slate-700 border-slate-700 text-slate-200'}"
        title="Inställningar"
      >
        <Settings class="w-3.5 h-3.5 {$settings.theme === 'light' ? 'text-slate-600' : 'text-slate-300'}" />
        <span class="hidden sm:inline">Inställningar</span>
      </button>

      <!-- Refresh Panes Button -->
      <button
        on:click={() => {
          refreshPane("left");
          refreshPane("right");
        }}
        class="border px-2.5 py-1.5 rounded-md text-xs font-mono flex items-center gap-1 transition-all
          {$settings.theme === 'light'
            ? 'bg-white hover:bg-slate-100 border-slate-300 text-slate-700'
            : 'bg-slate-800 hover:bg-slate-700 border-slate-700 text-slate-200'}"
        title="Uppdatera båda paneler"
      >
        <RefreshCw class="w-3.5 h-3.5 opacity-70" />
      </button>
    </div>
  </header>

  <!-- Notification Banner -->
  {#if transferNotification}
    <div
      class="bg-blue-950/90 border-b border-blue-800/80 px-4 py-1.5 flex items-center justify-between text-xs font-mono text-blue-200 animate-fadeIn"
    >
      <div class="flex items-center gap-2">
        {#if isTransferring}
          <RefreshCw class="w-3.5 h-3.5 animate-spin text-blue-400" />
        {:else}
          <CheckCircle2 class="w-3.5 h-3.5 text-emerald-400" />
        {/if}
        <span>{transferNotification}</span>
      </div>
      {#if $activeTransfer}
        <div class="text-[11px] text-blue-300 font-bold">
          {(($activeTransfer.bytes_transferred / (1024 * 1024)).toFixed(1))} MB / {(($activeTransfer.total_bytes / (1024 * 1024)).toFixed(1))} MB ({($activeTransfer.speed_bytes_per_sec / (1024 * 1024)).toFixed(1)} MB/s)
        </div>
      {/if}
    </div>
  {/if}

  <!-- Toast Notification Message Popup -->
  {#if $toastMessage}
    <div
      class="fixed bottom-14 right-6 z-50 bg-slate-900 border border-slate-700 text-slate-100 px-4 py-2.5 rounded-lg shadow-2xl flex items-center gap-2 font-mono text-xs animate-bounce"
    >
      <CheckCircle2 class="w-4 h-4 text-emerald-400 flex-shrink-0" />
      <span>{$toastMessage}</span>
    </div>
  {/if}

  <!-- Main Dual-Pane Viewport with Draggable Splitter -->
  <main
    bind:this={mainContainerRef}
    class="flex-1 flex p-2 gap-1 overflow-hidden transition-colors
      {$settings.theme === 'light' ? 'bg-slate-100' : 'bg-slate-950'}"
  >
    <div style="width: {splitRatio}%; flex: none; height: 100%;" class="min-w-0">
      <Pane state={$leftPane} paneId="left" />
    </div>

    <Splitter
      isDragging={isDraggingSplitter}
      on:startResize={handleStartResize}
      on:resetResize={handleResetResize}
    />

    <div style="width: calc({100 - splitRatio}% - 0.5rem); flex: 1 1 0%; height: 100%;" class="min-w-0">
      <Pane state={$rightPane} paneId="right" />
    </div>
  </main>

  <!-- Bottom Function Key Toolbar -->
  <Toolbar
    onView={triggerView}
    onEdit={triggerEdit}
    onCopy={() => triggerCopy(false)}
    onMove={() => triggerCopy(true)}
    onCreateDir={() => showCreateDirModal.set(true)}
    onDelete={triggerDelete}
  />

  <!-- Modals -->
  <FileViewerModal />
  <FileEditorModal />
  <SearchModal />
  <CreateDirModal />
  <DeleteConfirmModal />
  <SftpModal />
  <SettingsModal />
  <ZipModal />
</div>
