<script lang="ts">
  import { showCreateDirModal, activePaneState, activePane, refreshPane } from "../stores/commander";
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
    <div class="bg-slate-900 border border-slate-700 rounded-xl w-full max-w-md shadow-2xl p-5 text-slate-100 font-sans">
      <div class="flex items-center justify-between pb-3 border-b border-slate-800">
        <div class="flex items-center gap-2 font-bold text-lg text-blue-400">
          <FolderPlus class="w-5 h-5" />
          <span>F7: Skapa Ny Mapp</span>
        </div>
        <button on:click={() => showCreateDirModal.set(false)} class="text-slate-400 hover:text-white">
          <X class="w-5 h-5" />
        </button>
      </div>

      <form on:submit|preventDefault={handleCreate} class="space-y-4 mt-4 font-mono text-xs">
        <div>
          <label class="block text-slate-400 mb-1">Mappnamn</label>
          <input
            type="text"
            bind:value={folderName}
            placeholder="ny_mapp"
            required
            autofocus
            class="w-full bg-slate-950 border border-slate-700 rounded px-3 py-2 text-slate-100 outline-none focus:border-blue-500"
          />
        </div>

        <div class="text-xs text-slate-500">
          Mappen skapas i: <span class="text-slate-300 font-bold">{$activePaneState.currentPath}</span>
        </div>

        <div class="flex justify-end gap-2 pt-3 border-t border-slate-800">
          <button
            type="button"
            on:click={() => showCreateDirModal.set(false)}
            class="px-4 py-2 rounded bg-slate-800 hover:bg-slate-700 text-slate-300 font-semibold"
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
