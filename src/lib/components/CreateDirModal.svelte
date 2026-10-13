<script lang="ts">
  import { showCreateDirModal, activePaneState, activePane, refreshPane } from "../stores/commander";
  import { settings } from "../stores/settings";
  import { createDirectory } from "../tauri";
  import { FolderPlus, X } from "lucide-svelte";

  let folderName = "";

  async function handleCreate() {
    if (!folderName.trim()) return;
    const targetPath = `${$activePaneState.currentPath}/${folderName.trim()}`.replace("//", "/");
    try {
      await createDirectory(targetPath, $activePaneState.sftpConfig);
      refreshPane($activePane);
      showCreateDirModal.set(false);
      folderName = "";
    } catch (e: any) {
      alert(`Kunde inte skapa mappen:\n${e?.toString()}`);
    }
  }
</script>

{#if $showCreateDirModal}
  <div class="fixed inset-0 bg-black/70 backdrop-blur-sm z-50 flex items-center justify-center p-4">
    <div
      class="border rounded-xl w-full max-w-md shadow-2xl p-5 font-sans transition-colors
        {$settings.theme === 'light' ? 'bg-white border-slate-300 text-slate-800' : 'bg-slate-900 border-slate-700 text-slate-100'}"
    >
      <div class="flex items-center justify-between pb-3 border-b {$settings.theme === 'light' ? 'border-slate-200' : 'border-slate-800'}">
        <div class="flex items-center gap-2 font-bold text-lg text-blue-500">
          <FolderPlus class="w-5 h-5" />
          <span>F7: Skapa Ny Mapp</span>
        </div>
        <button
          on:click={() => showCreateDirModal.set(false)}
          class="{$settings.theme === 'light' ? 'text-slate-400 hover:text-slate-700' : 'text-slate-400 hover:text-white'}"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <form on:submit|preventDefault={handleCreate} class="space-y-4 mt-4 font-mono text-xs">
        <div>
          <label for="create-folder-input" class="block mb-1 {$settings.theme === 'light' ? 'text-slate-600 font-sans' : 'text-slate-400 font-sans'}">
            Mappnamn
          </label>
          <input
            id="create-folder-input"
            type="text"
            bind:value={folderName}
            placeholder="ny_mapp"
            required
            class="w-full border rounded px-3 py-2 outline-none focus:border-blue-500 font-mono
              {$settings.theme === 'light' ? 'bg-white border-slate-300 text-slate-800' : 'bg-slate-950 border-slate-700 text-slate-100'}"
          />
        </div>

        <div class="text-xs {$settings.theme === 'light' ? 'text-slate-500' : 'text-slate-400'}">
          Mappen skapas i: <span class="font-bold {$settings.theme === 'light' ? 'text-slate-800' : 'text-slate-200'}">{$activePaneState.currentPath}</span>
        </div>

        <div class="flex justify-end gap-2 pt-3 border-t {$settings.theme === 'light' ? 'border-slate-200' : 'border-slate-800'}">
          <button
            type="button"
            on:click={() => showCreateDirModal.set(false)}
            class="px-4 py-2 rounded font-semibold transition-colors
              {$settings.theme === 'light' ? 'bg-slate-100 hover:bg-slate-200 text-slate-700' : 'bg-slate-800 hover:bg-slate-700 text-slate-300'}"
          >
            Avbryt
          </button>
          <button
            type="submit"
            class="px-4 py-2 rounded bg-blue-600 hover:bg-blue-500 text-white font-semibold shadow-lg shadow-blue-600/30"
          >
            Skapa Mapp
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}
