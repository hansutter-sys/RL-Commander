<script lang="ts">
  import { showZipModal, zipSourcePaths, zipDefaultName, activePaneState, activePane, refreshPane } from "../stores/commander";
  import { settings } from "../stores/settings";
  import { zipItems } from "../tauri";
  import { FolderArchive, X, Archive, RefreshCw } from "lucide-svelte";

  let zipName = "";
  let isCompressing = false;
  let errorMsg: string | null = null;

  $: if ($showZipModal) {
    zipName = $zipDefaultName || "arkiv.zip";
    errorMsg = null;
  }

  async function handleCompress() {
    if (!zipName.trim() || $zipSourcePaths.length === 0) return;
    
    let finalZipName = zipName.trim();
    if (!finalZipName.toLowerCase().endsWith(".zip")) {
      finalZipName += ".zip";
    }

    const currentDir = $activePaneState.currentPath;
    const destZipPath = `${currentDir}/${finalZipName}`.replace("//", "/");

    isCompressing = true;
    errorMsg = null;

    try {
      await zipItems($zipSourcePaths, destZipPath, $activePaneState.sftpConfig);
      refreshPane("left");
      refreshPane("right");
      showZipModal.set(false);
    } catch (err: any) {
      errorMsg = `Fel vid komprimering: ${err?.toString()}`;
    } finally {
      isCompressing = false;
    }
  }

  function handleClose() {
    if (!isCompressing) {
      showZipModal.set(false);
    }
  }
</script>

{#if $showZipModal}
  <div class="fixed inset-0 bg-black/75 backdrop-blur-sm z-50 flex items-center justify-center p-4">
    <div
      class="border rounded-xl w-full max-w-md shadow-2xl p-5 font-sans transition-colors
        {$settings.theme === 'light' ? 'bg-white border-slate-300 text-slate-800' : 'bg-slate-900 border-slate-700 text-slate-100'}"
    >
      <!-- Header -->
      <div class="flex items-center justify-between pb-3 border-b {$settings.theme === 'light' ? 'border-slate-200' : 'border-slate-800'}">
        <div class="flex items-center gap-2 font-bold text-lg text-amber-500">
          <FolderArchive class="w-5 h-5 text-amber-500" />
          <span>Komprimera till ZIP</span>
        </div>
        <button
          on:click={handleClose}
          class="transition-colors {$settings.theme === 'light' ? 'text-slate-400 hover:text-slate-700' : 'text-slate-400 hover:text-white'}"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- Content Form -->
      <form on:submit|preventDefault={handleCompress} class="space-y-4 mt-4 font-mono text-xs">
        <div>
          <label for="zip-name-input" class="block mb-1 font-semibold {$settings.theme === 'light' ? 'text-slate-700' : 'text-slate-300'}">
            Namn på ZIP-arkiv
          </label>
          <input
            id="zip-name-input"
            type="text"
            bind:value={zipName}
            placeholder="arkiv.zip"
            required
            disabled={isCompressing}
            class="w-full border rounded px-3 py-2 outline-none focus:border-amber-500 font-mono disabled:opacity-50
              {$settings.theme === 'light' ? 'bg-white border-slate-300 text-slate-800' : 'bg-slate-950 border-slate-700 text-slate-100'}"
          />
        </div>

        <div
          class="p-2.5 rounded border space-y-1
            {$settings.theme === 'light' ? 'bg-slate-50 border-slate-200' : 'bg-slate-950/60 border-slate-800'}"
        >
          <div class="{$settings.theme === 'light' ? 'text-slate-600' : 'text-slate-400'}">
            Sparas i: <span class="font-semibold truncate block {$settings.theme === 'light' ? 'text-slate-900' : 'text-slate-200'}">{$activePaneState.currentPath}</span>
          </div>
          <div class="{$settings.theme === 'light' ? 'text-slate-600' : 'text-slate-400'}">
            Objekt att komprimera: <span class="text-amber-500 font-bold">{$zipSourcePaths.length} st</span>
          </div>
          {#if $zipSourcePaths.length > 0}
            <div
              class="max-h-24 overflow-y-auto text-[11px] mt-1 border-t pt-1 space-y-0.5
                {$settings.theme === 'light' ? 'border-slate-200 text-slate-500' : 'border-slate-800/80 text-slate-400'}"
            >
              {#each $zipSourcePaths as path}
                <div class="truncate {$settings.theme === 'light' ? 'text-slate-700' : 'text-slate-300'}">• {path.split('/').pop()}</div>
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
        <div class="flex justify-end gap-2 pt-3 border-t {$settings.theme === 'light' ? 'border-slate-200' : 'border-slate-800'}">
          <button
            type="button"
            on:click={handleClose}
            disabled={isCompressing}
            class="px-4 py-2 rounded font-semibold transition-colors disabled:opacity-50
              {$settings.theme === 'light' ? 'bg-slate-100 hover:bg-slate-200 text-slate-700' : 'bg-slate-800 hover:bg-slate-700 text-slate-300'}"
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
