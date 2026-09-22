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
  import { Cpu, ArrowLeftRight, CheckCircle2, RefreshCw, Server, Search, Settings } from "lucide-svelte";

  let isTransferring = false;
  let transferNotification: string | null = null;

  onMount(() => {
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

    // Alt+F7 Search
    if (e.altKey && e.key === "F7") {
      e.preventDefault();
      showSearchModal.set(true);
      return;
    }

    const currentPaneId = $activePane;
    const currentStore = currentPaneId === "left" ? leftPane : rightPane;
    const state = $activePaneState;

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

<div class="h-screen w-screen bg-slate-950 text-slate-100 flex flex-col overflow-hidden select-none font-sans">
  <!-- Top Application Bar -->
  <header class="bg-slate-900/90 border-b border-slate-800 px-4 py-2.5 flex items-center justify-between shadow-md">
    <div class="flex items-center gap-3">
      <div class="w-8 h-8 rounded-lg bg-gradient-to-tr from-blue-600 via-indigo-500 to-purple-600 flex items-center justify-center shadow-lg shadow-blue-500/20">
        <Cpu class="w-5 h-5 text-white" />
      </div>
      <div>
        <h1 class="font-black text-base tracking-wider bg-gradient-to-r from-blue-400 via-indigo-300 to-purple-400 bg-clip-text text-transparent">
          RL COMMANDER <span class="text-xs font-mono font-normal text-slate-400">v1.2</span>
        </h1>
        <p class="text-[10px] font-mono text-slate-400 -mt-0.5">High Performance Dual-Pane & SFTP Engine</p>
      </div>
    </div>

    <!-- Active Pane Status Indicator -->
    <div class="hidden sm:flex items-center gap-3 bg-slate-950 px-3 py-1.5 rounded-lg border border-slate-800 text-xs font-mono">
      <div class="flex items-center gap-1.5">
        <span class="w-2 h-2 rounded-full bg-emerald-500 animate-pulse"></span>
        <span class="text-slate-400">Aktiv Pane:</span>
        <span class="font-bold text-blue-400 uppercase">{$activePane}</span>
      </div>
      <div class="h-3 w-px bg-slate-800"></div>
      <button
        on:click={toggleActivePane}
        class="text-slate-400 hover:text-white flex items-center gap-1 transition-colors hover:bg-slate-900 px-1.5 py-0.5 rounded"
      >
        <ArrowLeftRight class="w-3.5 h-3.5 text-indigo-400" />
        <span>Växla (Tab)</span>
      </button>
    </div>

    <div class="flex items-center gap-2">
      <button
        on:click={() => showSearchModal.set(true)}
        class="bg-indigo-950/60 hover:bg-indigo-900 border border-indigo-800/80 text-indigo-300 px-3 py-1.5 rounded-md text-xs font-mono font-semibold flex items-center gap-1.5 transition-all shadow-sm"
      >
        <Search class="w-3.5 h-3.5 text-indigo-400" />
        <span>Sök (Alt+F7)</span>
      </button>
      <button
        on:click={() => showSftpModal.set(true)}
        class="bg-purple-950/60 hover:bg-purple-900 border border-purple-800/80 text-purple-300 px-3 py-1.5 rounded-md text-xs font-mono font-semibold flex items-center gap-1.5 transition-all shadow-sm"
      >
        <Server class="w-3.5 h-3.5 text-purple-400" />
        <span>SFTP Manager</span>
      </button>
      <button
        on:click={() => showSettingsModal.set(true)}
        class="bg-slate-800 hover:bg-slate-700 border border-slate-700 text-slate-200 px-2.5 py-1.5 rounded-md text-xs font-mono font-semibold flex items-center gap-1.5 transition-all shadow-sm"
        title="Inställningar"
      >
        <Settings class="w-3.5 h-3.5 text-slate-300" />
        <span class="hidden sm:inline">Inställningar</span>
      </button>
      <button
        on:click={() => {
          refreshPane("left");
          refreshPane("right");
        }}
        class="bg-slate-800 hover:bg-slate-700 border border-slate-700 text-slate-200 px-2.5 py-1.5 rounded-md text-xs font-mono flex items-center gap-1 transition-all"
        title="Uppdatera båda paneler"
      >
        <RefreshCw class="w-3.5 h-3.5 text-slate-400" />
      </button>
    </div>
  </header>

  <!-- Notification Banner -->
  {#if transferNotification}
    <div class="bg-blue-950/90 border-b border-blue-800/80 px-4 py-1.5 flex items-center justify-between text-xs font-mono text-blue-200 animate-fadeIn">
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

  <!-- Main Dual-Pane Viewport -->
  <main class="flex-1 flex p-2 gap-2 overflow-hidden bg-slate-950">
    <Pane state={$leftPane} paneId="left" />
    <Splitter />
    <Pane state={$rightPane} paneId="right" />
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
