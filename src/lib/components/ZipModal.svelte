<script lang="ts">
  import { showZipModal, zipSourcePaths, zipDefaultName, activePaneState, activePane, refreshPane } from "../stores/commander";
  import { zipItems } from "../tauri";
  import { Archive, X, FolderArchive, CheckCircle2, RefreshCw } from "lucide-svelte";

  let zipName = "";
  let isCompressing = false;
  let errorMsg = "";

  $: if ($showZipModal && $zipDefaultName) {
    zipName = $zipDefaultName;
    errorMsg = "";
    isCompressing = false;
  }

  async function handleCompress() {
    if (!zipName.trim() || $zipSourcePaths.length === 0) return;
    
    let targetFileName = zipName.trim();
    if (!targetFileName.toLowerCase().endsWith(".zip")) {
      targetFileName += ".zip";
    }

    const currentDir = $activePaneState.currentPath;
    const destZipPath = currentDir.endsWith("/")
      ? `${currentDir}${targetFileName}`
      : `${currentDir}/${targetFileName}`;

    isCompressing = true;
    errorMsg = "";

    try {
      await zipItems($zipSourcePaths, destZipPath, $activePaneState.sftpConfig);
      refreshPane("left");
      refreshPane("right");
      showZipModal.set(false);
    } catch (err: any) {
      errorMsg = err?.toString() || "Kommando misslyckades vid skapande av zip-arkiv.";
    } finally {
      isCompressing = false;
    }
  }

  function handleClose() {
    showZipModal.set(false);
  }
</script>

{#if $showZipModal}
  <div class="fixed inset-0 bg-black/75 backdrop-blur-sm z-50 flex items-center justify-center p-4">
    <div class="bg-slate-900 border border-slate-700 rounded-xl w-full max-w-md shadow-2xl p-5 text-slate-100 font-sans">
      <!-- Header -->
      <div class="flex items-center justify-between pb-3 border-b border-slate-800">
        <div class="flex items-center gap-2 font-bold text-lg text-amber-400">
          <FolderArchive class="w-5 h-5 text-amber-400" />
          <span>Komprimera till ZIP</span>
        </div>
        <button on:click={handleClose} class="text-slate-400 hover:text-white transition-colors">
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- Content Form -->
      <form on:submit|preventDefault={handleCompress} class="space-y-4 mt-4 font-mono text-xs">
        <div>
          <label class="block text-slate-400 mb-1 font-semibold">Namn på ZIP-arkiv</label>
          <input
            type="text"
            bind:value={zipName}
            placeholder="arkiv.zip"
            required
            disabled={isCompressing}
            autofocus
            class="w-full bg-slate-950 border border-slate-700 rounded px-3 py-2 text-slate-100 outline-none focus:border-amber-500 font-mono disabled:opacity-50"
          />
        </div>

        <div class="bg-slate-950/60 p-2.5 rounded border border-slate-800 space-y-1">
          <div class="text-slate-400">
            Sparas i: <span class="text-slate-200 font-semibold truncate block">{$activePaneState.currentPath}</span>
          </div>
          <div class="text-slate-400">
            Objekt att komprimera: <span class="text-amber-400 font-bold">{$zipSourcePaths.length} st</span>
          </div>
          {#if $zipSourcePaths.length > 0}
            <div class="max-h-24 overflow-y-auto text-[11px] text-slate-400 mt-1 border-t border-slate-800/80 pt-1 space-y-0.5">
              {#each $zipSourcePaths as path}
                <div class="truncate text-slate-300">• {path.split('/').pop()}</div>
              {/each}
            </div>
          {/if}
        </div>

        {#if errorMsg}
          <div class="p-2.5 rounded bg-red-950/80 border border-red-800 text-red-300 text-xs">
            {errorMsg}
          </div>
        {/if}

        <!-- Actions -->
        <div class="flex justify-end gap-2 pt-3 border-t border-slate-800">
          <button
            type="button"
            on:click={handleClose}
            disabled={isCompressing}
            class="px-4 py-2 rounded bg-slate-800 hover:bg-slate-700 text-slate-300 font-semibold disabled:opacity-50"
          >
            Avbryt
          </button>
          <button
            type="submit"
            disabled={isCompressing || !zipName.trim()}
            class="px-4 py-2 rounded bg-amber-600 hover:bg-amber-500 text-white font-semibold flex items-center gap-1.5 shadow-lg shadow-amber-600/30 disabled:opacity-50"
          >
            {#if isCompressing}
              <RefreshCw class="w-4 h-4 animate-spin text-white" />
              <span>Komprimerar...</span>
            {:else}
              <Archive class="w-4 h-4" />
              <span>Komprimera</span>
            {/if}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}
